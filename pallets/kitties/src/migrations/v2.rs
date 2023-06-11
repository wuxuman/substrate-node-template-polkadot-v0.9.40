use frame_support::{
    pallet_prelude::*,
    storage::StoragePrefixedMap,
    traits::GetStorageVersion,
    weights::Weight,
};
use frame_support::{migration::storage_key_iter ,Blake2_128Concat};

#[derive(Encode, Decode, Clone, Copy, Default,RuntimeDebug, PartialEq,Eq,TypeInfo, MaxEncodedLen)]
	pub struct KittyV1{
		pub dna:[u8;16],
		pub name:[u8;4],
	}

pub fn migrate<T:crate::Config>() ->Weight{
    let on_chain_version=crate::Pallet::<T>::on_chain_storage_version();
    let current_version=crate::Pallet::<T>::current_storage_version();

    if on_chain_version!=1{
        return Weight::zero();
    }

    if current_version!=2{
        return Weight::zero();
    }

    let module=crate::Kitties::<T>::module_prefix();
    let item=crate::Kitties::<T>::storage_prefix();
    let name_suffix:[u8;4]=*b"efgh";
    let mut name:[u8;8]=[0;8];

    for (index,kitty_v1) in storage_key_iter::<crate::KittyId, KittyV1,Blake2_128Concat>(module, item).drain(){
       
        name[..kitty_v1.name.len()].copy_from_slice(&kitty_v1.name);
        name[kitty_v1.name.len()..].copy_from_slice(&name_suffix);

        let new_kitty=crate::Kitty{
            dna:kitty_v1.dna,
            name,
        };
        crate::Kitties::<T>::insert(index,&new_kitty);
    }
    StorageVersion::new(2).put::<crate::Pallet<T>>();
    Weight::zero()

}