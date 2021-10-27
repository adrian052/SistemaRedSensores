use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::{near_bindgen, setup_alloc};
use near_sdk::{
    collections::{ UnorderedMap},
    json_types::{ U128,U64},
};

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
    id_estado: U128,
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
    id: U128,
    timestamp: U64, 
    estados: Vec<U128>,
    incidencia: Option<String> 
}


//Estructura principal del sistema de red de sensores.
#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize)]
pub struct SistemaRedSensores {
    racks: UnorderedMap<U64,Rack>,
    sensores: UnorderedMap<U64,Sensor>,
    estados: UnorderedMap<U128,Estado>,
    update_estado: Vec<ActualizacionEstado>
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
            update_estado: Vec::new(),
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
}