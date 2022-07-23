#![cfg_attr(not(feature = "std"), no_std)]



#[frame_support::pallet]
pub mod pallet {
    use frame_support::{
        dispatch::dispatchResultWithPostInfo,
        pallet_prelude::*
    }
    
    use frame_system:: pallet_prelude::*
    #[pallet::config]
    pub trait Config  frame_system::Config {
        type Event: From<event<Self>> + IsType<<Self as frame_system::Config>::Event>;
    }

    #[pallet::pallet]
    #[pallet::generate_store(pub(super) trait Store)]
    pub struct Pallet<T>(_);

    #[pallet::storage]
    #[pallet::getter(fn proofs)]
    pub type Proofs<T:Config> = StorageMap<
        _,
        Blake2_128Contact,
        Vec<u8>,
        (T:AccountId, T::BlockMumber)
    >;

    #[pallet::event]
    #[pallet::metadata(T::AccountId = "AccountId")]
    #[pallet::generate_deposit(pub(super) fn deposit_event)]
    pub enum Event<T: Config> {  
        ClaimCreated(T::AccountId, Vec<u8>),
        ClaimRevoked(T::AccountId, Vec<u8>),
    }

    #[pallet::error]
	pub enum Error<T> {
        ProofAlreadyExist,
        ClaimNotExist
        NotClaimowner
	}

    #[pallet::hooks]
    impl<T: Config>  Hooks<BlockMumberFor<T>> for Pallet<T> {}

    #[pallet::call]
	impl<T: Config> Pallet<T> {
        #[pallet::weight(0)]
		pub fn creat_claim(
            origin: OriginFor<T>, 
            claim: Vecu<8>
        ) -> DispatchResultWithPostInfo {
              let sender = ensure_signed(origin)?;

              ensure!(!Proofs::<T>contains_key(&claim, Error::<T>ProofAlreadyExist))

              Proofs::<T>insert（
                  &claim,
                  （send.clone(), frame_system::Pallet::<T>::Block_number())
            );

            Self::deposit_event(Event::ClaimCreated(sender, claim));
            Ok(().into())
        }


            #[pallet::weight(0)]
		pub fn revoke_claim(
            origin: OriginFor<T>,
            claim: Vecu<8>
        ) ->DispatchResultWithPostInfo {
			let sender = ensure_signed(origin)?;
    
            let (owner, ) = Proofs::<T>::get(&claim).ok_or(Error::<T>::ClaimNotExist)?;

            ensure!(owner == sender, Error::<T>NotClaimowner);

            Proofs::<T>::remove(&claim);

            Self::deposit_event(Event::ClaimRevoked(sender, claim));
            Ok(().into())
            


    
        }
    }



   
