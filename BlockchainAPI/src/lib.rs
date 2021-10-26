use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::{env, near_bindgen, setup_alloc};
use near_sdk::{
    collections::{ UnorderedMap,LookupMap },
    AccountId, PublicKey, Balance, Promise,
    json_types::{ U128, Base58PublicKey,U64},
};

setup_alloc!();

/*Estructuras auxiliares*/ 
#[derive(Debug, BorshDeserialize, BorshSerialize)]
pub enum TipoSensor {
    Ph, 
    HumedadRelativa,
    OnOff,
    Temperatura, 
}

#[derive(Debug, BorshDeserialize, BorshSerialize)]
pub struct Rack {
    id: U64,
    descripcion: String,
}



#[derive(Debug, BorshDeserialize, BorshSerialize)]
pub struct Sensor{
    id: U64,
    id_rack: U64,
    tipo: TipoSensor,
    descripcion: String,
}

#[derive(Debug, BorshDeserialize, BorshSerialize)]
pub struct Estado{
    id_estado: U128,
    id_sensor: U64,
    valor: String,
}

#[derive(Debug, BorshDeserialize, BorshSerialize)]
pub struct ActualizacionEstado{
    id: U128,
    timestamp: U64, 
    estados: Vec<U128>,
    incidencia: Option<String> 
}


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
    #[init]
    pub fn new() -> Self {
        Self {
            racks: UnorderedMap::new(b"".to_vec()),
            sensores: UnorderedMap::new(b"".to_vec()),
            estados: UnorderedMap::new(b"".to_vec()),
            update_estado: Vec::new(),
        }
    }

    pub fn nuevo_rack(&mut self, id:U64, descripcion:String){
        //verificar el id del rack
        assert!(self.racks.get(&id).is_none(),"El id del rack ya está registrado");
        let nuevo_rack = Rack {
            id,
            descripcion,
        };
        self.racks.insert(&id, &nuevo_rack);
    }

    pub fn get_descripcion_rack(self,id:U64) -> String{
        match self.racks.get(&id) {
            None => String::from("None"),
            Some(r) => r.descripcion
        }
    }

    pub fn agregar_sensor(&mut self, id:U64,id_rack:U64,tipo_sensor:String,descripcion:String){
        //verificar el id del rack.
        assert!(self.racks.get(&id_rack).is_none(),"El id del rack no está registrado");
        //verificar el id del sensor.
        assert!(!self.sensores.get(&id).is_none(),"El id del sensor ya está registrado");
        //verificar el tipo.
        let tipo:TipoSensor;
        match tipo_sensor.as_str() {
            "PH" => tipo = TipoSensor::Ph,
            "HUMEDAD_RELATIVA"=> tipo = TipoSensor::HumedadRelativa,
            "ON_OFF" => tipo = TipoSensor::OnOff,
            "TEMPERATURA" => tipo =TipoSensor::Temperatura,
            _=>panic!("Elije un tipo de sensor valido"),
        };

        let nuevo_sensor = Sensor{
            id,
            id_rack,
            tipo,
            descripcion,
        };

        self.sensores.insert(&id,&nuevo_sensor);
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use near_sdk::MockedBlockchain;
    use near_sdk::{testing_env, VMContext};


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
        contract.nuevo_rack(U64(0),String::from("Test 1"));
        assert_eq!(contract.get_descripcion_rack(U64(0)),String::from("Test 1"));
    }


    #[test]
    #[should_panic(expected="El id del rack ya está registrado")]
    fn agregar_rack_id_repetido(){
        let context = get_context(vec![], false);
        testing_env!(context);
        let mut contract = SistemaRedSensores::new();
        contract.nuevo_rack(U64(0),String::from("Test 1"));
        contract.nuevo_rack(U64(0),String::from("Test 2"));

    }

    #[test]
    fn agregar_un_sensor(){
        let context = get_context(vec![], false);
        testing_env!(context);
    }
    

}