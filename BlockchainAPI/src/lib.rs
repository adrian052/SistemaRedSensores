use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::{near_bindgen, setup_alloc};
use near_sdk::{
    collections::{ UnorderedMap,UnorderedSet},
    json_types::{U64,I64},
};
use chrono::{Utc};
setup_alloc!();


//Estructuras auxiliares

/**
 * Enum de tipo sensores
 * Se definen los tipos de sensores
 * es posible agregar mas tipos posteriormente.
*/
#[derive(Debug, BorshDeserialize, BorshSerialize)]
#[derive(Copy, Clone)]
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
    timestamp: I64, 
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
    actualizaciones_estado: Vec<ActualizacionEstado>
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
        }
    }

    /** Inserta un nuevo rack en la estructura map de racks con validaciones */
    pub fn nuevo_rack(&mut self, id:U64, descripcion:String){
        //valida el id del rack
        assert!(self.racks.get(&id).is_none(),"El id del rack ya está registrado");
        let nuevo_rack = Rack {
            id,
            descripcion,
        };
        self.racks.insert(&id, &nuevo_rack);
    }

    /*Inserta un nuevo sensor en la estructura map de sensores con validaciones*/
    pub fn nuevo_sensor(&mut self, id:U64,id_rack:U64,tipo_sensor:String,descripcion:String){
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
    }

    

    /**
     * Funcion de actualizar estado
     * Funcion donde ingresas un vector strings de la forma <id>:<valor>
     * y agregar el valor a la actualizacion de estados.
     */
    pub fn actualizar_estado(&mut self,estados_string:Vec<String>){
        let mut estados_number:Vec<U64>  = Vec::new();
        let mut sensores_actualizados: UnorderedSet<U64> = UnorderedSet::new(b"sensores_actualizados".to_vec());
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
            self.estados.insert(&U64(self.estados.len()), &nuevo_estado);
            estados_number.push(U64(self.estados.len()));
        }
        let curr_size = self.estados.len();

        let nueva_actualizacion = ActualizacionEstado {
            id: U64(curr_size),
            timestamp : I64(Utc::now().timestamp()), 
            estados: estados_number,
            incidencia: None,
        };
        self.actualizaciones_estado.push(nueva_actualizacion);
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
            return self.actualizaciones_estado[self.actualizaciones_estado.len()-1].timestamp;
        }    
    }
}

//Modulo de test unitarios.
#[cfg(test)]
mod tests {
    use super::*;
    use near_sdk::MockedBlockchain;
    use near_sdk::{testing_env, VMContext};

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
        contract.actualizar_estado(vec![str("0:10"),str("1:10.0"),str("2:true"),str("3:25.3")]);
        //validamos que la ultima actualizacion sea ahora
        let ahora:I64 = I64(Utc::now().timestamp());
        assert_eq!(contract.get_time_ultima_actualizacion(),ahora);
    }

    #[test]
    #[should_panic(expected="No se puede parsear el valor de Temperatura")]
    fn agregar_actualiazcion_temperatura_invalida(){
        let context = get_context(vec![], false);
        testing_env!(context);
        let mut contract = SistemaRedSensores::new();
        contract.nuevo_rack(U64(0),str("Test 1"));
        contract.nuevo_sensor(U64(3),U64(0),str("TEMPERATURA"),str("Sensor para la parte baja"));
        contract.actualizar_estado(vec![str("3:false")]);
    }

    #[test]
    #[should_panic(expected="No se puede parsear el valor de OnOff")]
    fn agregar_actualizacion_onoff_invalido(){
        let context = get_context(vec![], false);
        testing_env!(context);
        let mut contract = SistemaRedSensores::new();
        contract.nuevo_rack(U64(0),str("Test 1"));
        contract.nuevo_sensor(U64(5),U64(0),str("ON_OFF"),str("Sensor para la parte baja"));
        contract.actualizar_estado(vec![str("5:10.3")]);
    }

    #[test]
    #[should_panic(expected="No se puede parsear el valor de HumedadRelativa")]
    fn agregar_actualizacion_hr_invalido(){
        let context = get_context(vec![], false);
        testing_env!(context);
        let mut contract = SistemaRedSensores::new();
        contract.nuevo_rack(U64(100),str("Test 1"));
        contract.nuevo_sensor(U64(3),U64(100),str("HUMEDAD_RELATIVA"),str("Sensor para la parte baja"));
        contract.actualizar_estado(vec![str("3:false")]);
    }


    #[test]
    #[should_panic(expected="No se puede parsear el valor de Ph")]
    fn agregar_actualizacion_ph_invalido(){
        let context = get_context(vec![], false);
        testing_env!(context);
        let mut contract = SistemaRedSensores::new();
        contract.nuevo_rack(U64(99),str("Test 1"));
        contract.nuevo_sensor(U64(343),U64(99),str("PH"),str("Sensor para la parte baja"));
        contract.actualizar_estado(vec![str("343:10.3")]);
    }

    #[test]
    #[should_panic(expected="No se puede agregar dos actualizaciones del mismo sensor.")]
    fn agregar_dos_estados_mismo_sensor(){
        let context = get_context(vec![], false);
        testing_env!(context);
        let mut contract = SistemaRedSensores::new();
        contract.nuevo_rack(U64(99),str("Test 1"));
        contract.nuevo_sensor(U64(10),U64(99),str("PH"),str("Sensor para la parte baja"));
        contract.actualizar_estado(vec![str("10:10"),str("10:12")]);
    }
}