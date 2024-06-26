use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::{near_bindgen, setup_alloc};
use near_sdk::{
    collections::{ UnorderedMap,UnorderedSet},
    json_types::{U64,I64},
};
use std::collections::BTreeMap;




setup_alloc!();


//Estructuras auxiliares

/**
 * Enum de tipo sensores
 * Se definen los tipos de sensores
 * es posible agregar mas tipos posteriormente.
*/
#[derive(Debug, BorshDeserialize, BorshSerialize)]
#[derive(Copy, Clone,PartialEq)]
pub enum TipoSensor {
    Ph, 
    HumedadRelativa,
    OnOff,
    Temperatura, 
}

/**
 * Estructura rack
 * Un rack sera contiene un conjunto de senores de diferente tipo.
*/
#[derive(Debug, BorshDeserialize, BorshSerialize)]
pub struct Rack {
    id: U64,
    descripcion: String,
}

/**
 * Estructura sensor
 * Cada sensor está asociado a un unico rack y tienen un tipo especifico.
*/
#[derive(Debug, BorshDeserialize, BorshSerialize)]
pub struct Sensor{
    id: U64,
    id_rack: U64,
    tipo: TipoSensor,
    descripcion: String,
}


/**
 * Estructura estado
 * Es la parte estructura minima para el control de estado,
 * se construye para indicar un valor a un sensor en especifico.
*/
#[derive(Debug, BorshDeserialize, BorshSerialize)]
pub struct Estado{
    id_estado: U64,
    id_sensor: U64,
    valor: String,
}

/**
 * Estructura actualizacion estado
 * Se utiliza par encapsular una serie de estados
 * sondeados en un tiempo en comun.
*/
#[derive(Debug, BorshDeserialize, BorshSerialize)]
pub struct ActualizacionEstado{
    id: U64,
    timestamp: i64, 
    estados: Vec<U64>,
    incidencia: Option<String> 
}


//Estructura principal del sistema de red de sensores.
#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize)]
pub struct SistemaRedSensores {
    racks: UnorderedMap<U64,Rack>,
    sensores: UnorderedMap<U64,Sensor>,
    estados: UnorderedMap<U64,Estado>,
    actualizaciones_estado: Vec<ActualizacionEstado>,
    number_hash: i128
}

impl Default for SistemaRedSensores {
    fn default() -> Self {
        panic!("Debe inicializarse antes de usarse.");
    }
}

#[near_bindgen]
impl SistemaRedSensores {
    /** 
     * Constructor para SistemaRedSensores, inicializa el sistema con estructuras
     * de datos vacias.
    */
    #[init]
    pub fn new() -> Self {
        Self {
            /*Nota: cada uno de las estructuras de datos llevan un identificador unico*/
            racks: UnorderedMap::new(b"racks".to_vec()),
            sensores: UnorderedMap::new(b"sensores".to_vec()),
            estados: UnorderedMap::new(b"estados".to_vec()),
            actualizaciones_estado: Vec::new(),
            number_hash: 11,
        }
    }

    /** Inserta un nuevo rack en la estructura map de racks con validaciones */
    pub fn nuevo_rack(&mut self, id:U64, descripcion:String) -> bool {
        //valida el id del rack
        assert!(self.racks.get(&id).is_none(),"El id del rack ya está registrado");
        let nuevo_rack = Rack {
            id,
            descripcion,
        };
        self.racks.insert(&id, &nuevo_rack);
        return true;
    }

    /*Inserta un nuevo sensor en la estructura map de sensores con validaciones*/
    pub fn nuevo_sensor(&mut self, id:U64,id_rack:U64,tipo_sensor:String,descripcion:String) -> bool {
        //valida el id del rack.
        assert!(!self.racks.get(&id_rack).is_none(),"El id del rack no está registrado");
        //valida el id del sensor.
        assert!(self.sensores.get(&id).is_none(),"El id del sensor ya está registrado");
        //valida el tipo.
        let tipo:TipoSensor;
        match tipo_sensor.as_str() {
            "PH" => tipo = TipoSensor::Ph,
            "HUMEDAD_RELATIVA"=> tipo = TipoSensor::HumedadRelativa,
            "ON_OFF" => tipo = TipoSensor::OnOff,
            "TEMPERATURA" => tipo =TipoSensor::Temperatura,
            _=>panic!("Elije un tipo de sensor valido"),
        };

        let n_sensor = Sensor{
            id,
            id_rack,
            tipo,
            descripcion,
        };

        self.sensores.insert(&id,&n_sensor);
        return true;
    }

    

    /**
     * Funcion de actualizar estado
     * Funcion donde ingresas un vector strings de la forma <id>:<valor>
     * y agregar el valor a la actualizacion de estados.
     */
    pub fn actualizar_estado(&mut self,timestamp_update:i64,estados_string:Vec<String>) -> bool {
        let s: String = format!("{}{}","hash",self.number_hash);
        self.number_hash+=11;
        let mut estados_number:Vec<U64>  = Vec::new();
        let mut sensores_actualizados: UnorderedSet<U64> = UnorderedSet::new(s.as_bytes().to_vec());
        for cadena in estados_string.iter() {
            let values: Vec<&str> = cadena.split(':').collect();
            
            assert!(values.len()==2,"Numero de parametros invalido.");
            let id = values[0];
            assert!(!sensores_actualizados.contains(&U64(id.parse::<u64>().unwrap())),"No se puede agregar dos actualizaciones del mismo sensor.");
            sensores_actualizados.insert(&U64(id.parse::<u64>().unwrap()));
            let value = values[1];
            self.validar_contenido(id, value);
            //agregar el estado al arreglo de estados de blockchain y el ultimo numero a el estados number.
            let nuevo_estado =  Estado {
                id_estado : U64(self.estados.len()),
                id_sensor : U64(id.parse::<u64>().unwrap()),
                valor: String::from(value),
            };
            estados_number.push(U64(self.estados.len()));
            self.estados.insert(&U64(self.estados.len()), &nuevo_estado);
            
        }
        let curr_size = self.estados.len();

        let nueva_actualizacion = ActualizacionEstado {
            id: U64(curr_size),
            timestamp : timestamp_update, 
            estados: estados_number,
            incidencia: None,
        };
        self.actualizaciones_estado.push(nueva_actualizacion);
        return true;
    }

    
    /**
     * Funcion auxiliar para validar el contenido de un estado con valores strings.
     */
    fn validar_contenido(&self, id: &str,valor: &str){
        match id.parse::<u64>() {
            Ok(number) => match self.sensores.get(&U64(number)) {
                Some(sensor)=> match sensor.tipo {
                        TipoSensor::Ph => assert!(valor.parse::<u64>().is_ok(),"No se puede parsear el valor de Ph"), 
                        TipoSensor::HumedadRelativa => assert!(valor.parse::<f32>().is_ok(),"No se puede parsear el valor de HumedadRelativa"),
                        TipoSensor::OnOff => assert!(valor.parse::<bool>().is_ok(),"No se puede parsear el valor de OnOff"),
                        TipoSensor::Temperatura => assert!(valor.parse::<f32>().is_ok(),"No se puede parsear el valor de Temperatura"),
                    },
                None => panic!("El id del sensor ya está registrado."),
            },
            Err(_) => panic!("El formato del id no es correcto."),
        }
    }


    /**Geters*/
    pub fn get_descripcion_sensor(&self,id:U64) -> String{
        match self.sensores.get(&id) {
            None => String::from("None"),
            Some(r) => r.descripcion
        }
    }

    pub fn get_descripcion_rack(&self,id:U64) -> String{
        match self.racks.get(&id) {
            None => String::from("None"),
            Some(r) => r.descripcion
        }
    }

    pub fn get_time_ultima_actualizacion(&self) -> I64 {
        if self.actualizaciones_estado.len() == 0 {
            I64(0)
        }else{
            return I64(self.actualizaciones_estado[self.actualizaciones_estado.len()-1].timestamp);
        }    
    }

    pub fn get_historial(&self,tipo:String,init_timpstamp:i64,last_timpstamp:i64) -> BTreeMap<i64,BTreeMap<u64,String>>{
        let mut historial:BTreeMap<i64,BTreeMap<u64,String>> = BTreeMap::new();
        let tipo_sensor:TipoSensor;
        match tipo.as_ref() {
            "PH" => tipo_sensor = TipoSensor::Ph,
            "HUMEDAD_RELATIVA"=> tipo_sensor = TipoSensor::HumedadRelativa,
            "ON_OFF" => tipo_sensor = TipoSensor::OnOff,
            "TEMPERATURA" => tipo_sensor =TipoSensor::Temperatura,
            _=>panic!("Elije un tipo de sensor valido"),
        }

        for actualizacion in self.actualizaciones_estado.iter() {
            if actualizacion.timestamp>=init_timpstamp && actualizacion.timestamp<=last_timpstamp {
                let mut aux:BTreeMap<u64,String> = BTreeMap::new();
                for id_estado in actualizacion.estados.iter(){
                    let estado = self.estados.get(id_estado);
                    match estado {
                        Some(valor) => {
                            if self.sensores.get(&valor.id_sensor).unwrap().tipo == tipo_sensor{
                                let buf:u64 = valor.id_sensor.into();
                                aux.insert(buf,valor.valor);
                            }
                            
                        },
                        None => panic!("Error integridad: No se encuentra el estado registrado."),
                    }
                    
                }  
                if aux.len()>0{
                   historial.insert(actualizacion.timestamp,aux);      
                }
            }    
        }
        return historial;   
    }


    pub fn get_historial_sensor(&self,id:U64,init_timpstamp:i64,last_timpstamp:i64) -> BTreeMap<i64,String>{
        let mut historial = BTreeMap::new();
        for actualizacion in self.actualizaciones_estado.iter() {
            if actualizacion.timestamp>=init_timpstamp && actualizacion.timestamp<=last_timpstamp {
                for id_estado in actualizacion.estados.iter(){
                    let estado = self.estados.get(id_estado);
                    match estado {
                        Some(valor) => {
                            if valor.id_sensor == id{
                                historial.insert(actualizacion.timestamp,valor.valor);
                            }    
                        },
                        None => panic!("Error integridad: No se encuentra el estado registrado."),
                    }
                }    
            }
        }    
        return historial;
    }
    
    pub fn get_sensores_por_tipo(&self,tipo_sensor:String) -> Vec<U64> {
        let mut ans:Vec<U64> = vec!();
        let tipo:TipoSensor;
        match tipo_sensor.as_str() {
            "PH" => tipo = TipoSensor::Ph,
            "HUMEDAD_RELATIVA"=> tipo = TipoSensor::HumedadRelativa,
            "ON_OFF" => tipo = TipoSensor::OnOff,
            "TEMPERATURA" => tipo =TipoSensor::Temperatura,
            _=>panic!("Elije un tipo de sensor valido"),
        };

        for sensor in self.sensores.values() {
            if sensor.tipo == tipo {
                ans.push(sensor.id);
            }
        }
        return ans;
    }

    pub fn get_informacion_sensor(&self,id:U64) -> BTreeMap<String,String> {
        let mut informacion = BTreeMap::new();
        match self.sensores.get(&id){
            Some(sensor) => {
                let id_sensor:u64= sensor.id.into();
                let id_rack:u64= sensor.id_rack.into(); 
                informacion.insert(String::from("id_sensor"),id_sensor.to_string());
                informacion.insert(String::from("id_rack"),id_rack.to_string());
                informacion.insert(String::from("descripcion"),sensor.descripcion);
                match sensor.tipo {
                    TipoSensor::Ph => {informacion.insert(String::from("tipo"),String::from("ph"));}, 
                    TipoSensor::HumedadRelativa => {informacion.insert(String::from("tipo"),String::from("humedad_relativa"));},
                    TipoSensor::OnOff => {informacion.insert(String::from("tipo"),String::from("on_off"));},
                    TipoSensor::Temperatura => {informacion.insert(String::from("tipo"),String::from("temperatura"));},
                } 
                return informacion;
            },
            None => {return informacion;},
        }
    }

    pub fn get_lista_sensores(&self) -> Vec<BTreeMap<String,String>> {
        let mut answer_list = Vec::<BTreeMap::<String,String>>::new();
        for sensor in self.sensores.values() {
            let id_sensor:u64= sensor.id.into();
            let id_rack:u64= sensor.id_rack.into(); 
            let mut sensor_tree = BTreeMap::<String,String>::new();
            sensor_tree.insert(String::from("id"),id_sensor.to_string());
            sensor_tree.insert(String::from("id_rack"),id_rack.to_string());
            sensor_tree.insert(String::from("descripcion"),sensor.descripcion);
            match sensor.tipo {
                TipoSensor::Ph => {sensor_tree.insert(String::from("tipo"),String::from("ph"));}, 
                TipoSensor::HumedadRelativa => {sensor_tree.insert(String::from("tipo"),String::from("humedad_relativa"));},
                TipoSensor::OnOff => {sensor_tree.insert(String::from("tipo"),String::from("on_off"));},
                TipoSensor::Temperatura => {sensor_tree.insert(String::from("tipo"),String::from("temperatura"));},
            } 
            answer_list.insert(0,sensor_tree);
        }
        return answer_list;
    }

    pub fn get_lista_racks(&self) -> Vec<BTreeMap<String,String>> {
        let mut answer_list = Vec::<BTreeMap::<String,String>>::new();
        for rack in self.racks.values() {
            let id:u64= rack.id.into();
            let mut sensor_tree = BTreeMap::<String,String>::new();
            sensor_tree.insert(String::from("id"),id.to_string());
            sensor_tree.insert(String::from("descripcion"),rack.descripcion);
            
            answer_list.insert(0,sensor_tree);
        }
        return answer_list;
    }
}

//Modulo de test unitarios.
#[cfg(test)]
mod tests {
    use super::*;
    use near_sdk::MockedBlockchain;
    use near_sdk::{testing_env, VMContext};
    use chrono::Utc;

    fn str(input: &str) -> String {
        return String::from(input);
    }


    fn get_context(input: Vec<u8>, is_view: bool) -> VMContext {
        VMContext {
            current_account_id: "alice.testnet".to_string(),
            signer_account_id: "robert.testnet".to_string(),
            signer_account_pk: vec![0, 1, 2],
            predecessor_account_id: "jane.testnet".to_string(),
            input,
            block_index: 0,
            block_timestamp: 0,
            account_balance: 0,
            account_locked_balance: 0,
            storage_usage: 0,
            attached_deposit: 0,
            prepaid_gas: 10u64.pow(18),
            random_seed: vec![0, 1, 2],
            is_view,
            output_data_receivers: vec![],
            epoch_height: 19,
        }
    }

    #[test]
    fn agregar_un_rack() {
        let context = get_context(vec![], false);
        testing_env!(context);

        let mut contract = SistemaRedSensores::new();
        contract.nuevo_rack(U64(0),str("Test 1"));
        assert_eq!(contract.get_descripcion_rack(U64(0)),str("Test 1"));
    }


    #[test]
    #[should_panic(expected="El id del rack ya está registrado")]
    fn agregar_rack_id_repetido(){
        let context = get_context(vec![], false);
        testing_env!(context);
        let mut contract = SistemaRedSensores::new();
        contract.nuevo_rack(U64(0),str("Test 1"));
        contract.nuevo_rack(U64(0),str("Test 2"));
    }

    
    #[test]
    fn agregar_un_sensor(){
        let context = get_context(vec![], false);
        testing_env!(context);
        let mut contract = SistemaRedSensores::new();
        contract.nuevo_rack(U64(0),str("Test 1"));
        contract.nuevo_sensor(U64(0),U64(0),str("PH"),str("Descripcion"));
        assert_eq!(contract.get_descripcion_sensor(U64(0)),str("Descripcion"));
    }

    #[test]
    #[should_panic(expected="El id del sensor ya está registrado")]
    fn agregar_sensor_id_repetido(){
        let context = get_context(vec![], false);
        testing_env!(context);
        let mut contract = SistemaRedSensores::new();
        contract.nuevo_rack(U64(0),str("Test 1"));
        contract.nuevo_sensor(U64(0),U64(0),str("PH"),str("Descripcion"));
        contract.nuevo_sensor(U64(0),U64(0),str("PH"),str("Descripcion"));
    }
    #[test]
    #[should_panic(expected="El id del rack no está registrado")]
    fn agregar_sensor_rack_inexistente(){
        let context = get_context(vec![], false);
        testing_env!(context);
        let mut contract = SistemaRedSensores::new();
        contract.nuevo_sensor(U64(0),U64(0),str("PH"),str("Descripcion"));
    } 

    #[test]
    fn agregar_una_actualizacion(){
        let context = get_context(vec![], false);
        testing_env!(context);
        let mut contract = SistemaRedSensores::new();
        contract.nuevo_rack(U64(0),str("Test 1"));
        contract.nuevo_sensor(U64(0),U64(0),str("PH"),str("Sensor para la parte alta"));
        contract.nuevo_sensor(U64(1),U64(0),str("HUMEDAD_RELATIVA"),str("sensor colo rojo"));
        contract.nuevo_sensor(U64(2),U64(0),str("ON_OFF"),str("sensor de la parte norte"));
        contract.nuevo_sensor(U64(3),U64(0),str("TEMPERATURA"),str("Sensor para la parte baja"));
        let ahora:i64 = Utc::now().timestamp();
        contract.actualizar_estado(ahora,vec![str("0:10"),str("1:10.0"),str("2:true"),str("3:25.3")]);
        assert_eq!(contract.get_time_ultima_actualizacion(),I64(ahora));
    }

    #[test]
    #[should_panic(expected="No se puede parsear el valor de Temperatura")]
    fn agregar_actualiazcion_temperatura_invalida(){
        let context = get_context(vec![], false);
        testing_env!(context);
        let mut contract = SistemaRedSensores::new();
        contract.nuevo_rack(U64(0),str("Test 1"));
        contract.nuevo_sensor(U64(3),U64(0),str("TEMPERATURA"),str("Sensor para la parte baja"));
        let ahora:i64 = Utc::now().timestamp();
        contract.actualizar_estado(ahora,vec![str("3:false")]);
    }

    #[test]
    #[should_panic(expected="No se puede parsear el valor de OnOff")]
    fn agregar_actualizacion_onoff_invalido(){
        let context = get_context(vec![], false);
        testing_env!(context);
        let mut contract = SistemaRedSensores::new();
        contract.nuevo_rack(U64(0),str("Test 1"));
        contract.nuevo_sensor(U64(5),U64(0),str("ON_OFF"),str("Sensor para la parte baja"));
        let ahora:i64 = Utc::now().timestamp();
        contract.actualizar_estado(ahora,vec![str("5:10.3")]);
    }

    #[test]
    #[should_panic(expected="No se puede parsear el valor de HumedadRelativa")]
    fn agregar_actualizacion_hr_invalido(){
        let context = get_context(vec![], false);
        testing_env!(context);
        let mut contract = SistemaRedSensores::new();
        contract.nuevo_rack(U64(100),str("Test 1"));
        contract.nuevo_sensor(U64(3),U64(100),str("HUMEDAD_RELATIVA"),str("Sensor para la parte baja"));
        let ahora:i64 = Utc::now().timestamp();
        contract.actualizar_estado(ahora,vec![str("3:false")]);
    }


    #[test]
    #[should_panic(expected="No se puede parsear el valor de Ph")]
    fn agregar_actualizacion_ph_invalido(){
        let context = get_context(vec![], false);
        testing_env!(context);
        let mut contract = SistemaRedSensores::new();
        contract.nuevo_rack(U64(99),str("Test 1"));
        contract.nuevo_sensor(U64(343),U64(99),str("PH"),str("Sensor para la parte baja"));
        let ahora:i64 = Utc::now().timestamp();
        contract.actualizar_estado(ahora,vec![str("343:10.3")]);
    }

    #[test]
    #[should_panic(expected="No se puede agregar dos actualizaciones del mismo sensor.")]
    fn agregar_dos_estados_mismo_sensor(){
        let context = get_context(vec![], false);
        testing_env!(context);
        let mut contract = SistemaRedSensores::new();
        contract.nuevo_rack(U64(99),str("Test 1"));
        contract.nuevo_sensor(U64(10),U64(99),str("PH"),str("Sensor para la parte baja"));
        let ahora:i64 = Utc::now().timestamp();
        contract.actualizar_estado(ahora,vec![str("10:10"),str("10:12")]);
    }


    #[test]
    fn obtener_historial_temperatura(){
        let context = get_context(vec![], false);
        testing_env!(context);
        let mut contract = SistemaRedSensores::new();
        contract.nuevo_rack(U64(0),str("Test 1"));
        contract.nuevo_sensor(U64(0),U64(0),str("TEMPERATURA"),str("Descripcion"));
        contract.nuevo_sensor(U64(1),U64(0),str("HUMEDAD_RELATIVA"),str("Descripcion"));
        contract.actualizar_estado(0,vec![str("0:21.5")]);
        contract.actualizar_estado(1,vec![str("0:20.3")]);
        contract.actualizar_estado(2,vec![str("0:19.5")]);
        contract.actualizar_estado(3,vec![str("1:19.5")]);
        let ahora:i64 = Utc::now().timestamp();
        let historial = contract.get_historial(str("TEMPERATURA"),0,ahora);
        assert_eq!(historial.len(),3);
    }
}