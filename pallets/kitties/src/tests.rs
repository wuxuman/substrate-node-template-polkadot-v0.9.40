use crate::{mock::*,Error};
use frame_support::{assert_ok,assert_noop};


// #[test]
// fn it_works_for_create(){
//     new_test_ext().execute_with(||{
//         let kitty_id=0;
//         let account_id=1;

//         let _=Balances::set_balance(RuntimeOrigin::root(), account_id, 10_000_000, 0);

//         assert_eq!(KittiesModule::next_kitty_id(),kitty_id);
//         assert_ok!(KittiesModule::create(RuntimeOrigin::signed(account_id)));

//         assert_eq!(KittiesModule::next_kitty_id(),kitty_id+1);
//         assert_eq!(KittiesModule::kitties(kitty_id).is_some(),true);
//         assert_eq!(KittiesModule::kitty_owner(kitty_id),Some(account_id));
//         assert_eq!(KittiesModule::kitty_parents(kitty_id),None);

//         crate::NextKittyId::<Test>::set(crate::KittyId::max_value());
//         assert_noop!(
//             KittiesModule::create(RuntimeOrigin::signed(account_id)),
//             Error::<Test>::InvalidKittyId
//         );

//         // check whether create event emitted
//         //assert_eq!(System::events().len(), 1);

//         let kitty=KittiesModule::kitties(kitty_id).unwrap();

//         System::assert_has_event(crate::Event::<Test>::KittyCreated{who:account_id,kitty_id,kitty:kitty}.into());

//     })

// }

// #[test]
// fn it_works_for_breed(){
//     new_test_ext().execute_with(||{
//         let kitty_id=0;
//         let account_id=1;
//         let _=Balances::set_balance(RuntimeOrigin::root(), account_id, 10_000_000, 0);

//         assert_noop!(KittiesModule::breed(RuntimeOrigin::signed(1), kitty_id, kitty_id),
//                     Error::<Test>::SameKittyId);

//         assert_noop!(KittiesModule::breed(RuntimeOrigin::signed(1), kitty_id, kitty_id+1),
//                     Error::<Test>::InvalidKittyId);     

//         assert_ok!(KittiesModule::create(RuntimeOrigin::signed(account_id)));          
//         assert_ok!(KittiesModule::create(RuntimeOrigin::signed(account_id))); 

//         assert_eq!(KittiesModule::next_kitty_id(),kitty_id+2);

//         assert_ok!(KittiesModule::breed(RuntimeOrigin::signed(account_id),kitty_id,kitty_id+1));      

//         assert_eq!(KittiesModule::next_kitty_id(),kitty_id+3);

//         assert_eq!(KittiesModule::kitties(2).is_some(),true);

//         assert_eq!(KittiesModule::kitty_owner(2),Some(account_id));

//         assert_eq!(KittiesModule::kitty_parents(2),Some((kitty_id,kitty_id+1)));


//         // check whether breed event emitted
//        //assert_eq!(System::events().len(), 3);

//         let kitty=KittiesModule::kitties(kitty_id+2).unwrap();

//         System::assert_has_event(crate::Event::<Test>::KittyCreated{who:account_id,kitty_id,kitty:kitty}.into());
//         System::assert_has_event(crate::Event::<Test>::KittyCreated{who:account_id,kitty_id:kitty_id+1,kitty:kitty}.into());
//         System::assert_has_event(crate::Event::<Test>::KittyBreed {who:account_id,kitty_id:kitty_id+2,kitty:kitty}.into());


//     });
// }



// #[test]
// fn it_works_for_transfer(){
//     new_test_ext().execute_with(||{

//         let kitty_id=0;
//         let account_id=1;
//         let recipient_id=2;
//         let _=Balances::set_balance(RuntimeOrigin::root(), account_id, 10_000_000, 0);

//         assert_ok!(KittiesModule::create(RuntimeOrigin::signed(account_id)));

//         assert_noop!(KittiesModule::transfer(RuntimeOrigin::signed(10), recipient_id, kitty_id),
//                     Error::<Test>::NotOwner);

//         assert_ok!(KittiesModule::transfer(RuntimeOrigin::signed(account_id), recipient_id, kitty_id));

//         assert_eq!(KittiesModule::kitty_owner(kitty_id),Some(recipient_id));

//         assert_ok!(KittiesModule::transfer(RuntimeOrigin::signed(recipient_id), account_id, kitty_id));

//         assert_eq!(KittiesModule::kitty_owner(kitty_id),Some(account_id));

//       // check whether transfer event emitted
//       //assert_eq!(System::events().len(), 3);

//       let kitty=KittiesModule::kitties(kitty_id).unwrap();

//       System::assert_has_event(crate::Event::<Test>::KittyCreated{who:account_id,kitty_id,kitty:kitty}.into());
//       System::assert_has_event(crate::Event::<Test>::KittyTransferred {who:account_id,recipient: recipient_id,kitty_id:kitty_id}.into());
//       System::assert_has_event(crate::Event::<Test>::KittyTransferred {who:recipient_id,recipient: account_id,kitty_id:kitty_id}.into());

//     });
// }

// #[test]
// fn it_works_for_sale(){
//     new_test_ext().execute_with(||{
//         let kitty_id=0;
//         let account_id=1;
//         let _=Balances::set_balance(RuntimeOrigin::root(), account_id, 10_000_000, 0);


//         //check the validation that whether the kitty_id is in Kitties 
//         assert_noop!(KittiesModule::sale(RuntimeOrigin::signed(account_id),kitty_id),Error::<Test>::InvalidKittyId);

//         //create a kitty by account_id and kitty_id
//         assert_ok!(KittiesModule::create(RuntimeOrigin::signed(account_id)));

//         //check validation that whether owner is the kitty's owner validation
//         assert_noop!(KittiesModule::sale(RuntimeOrigin::signed(10),kitty_id),Error::<Test>::NotOwner);

//         //sale the kitty
//         assert_ok!(KittiesModule::sale(RuntimeOrigin::signed(account_id),kitty_id));

//         //check the validation whether the kitty already on sale
//         assert_noop!(KittiesModule::sale(RuntimeOrigin::signed(account_id),kitty_id),Error::<Test>::AlreadyOnSale);

//         //check whether the kitty_id inserted in KittyOnSale
//         assert_eq!(KittiesModule::kitty_on_sale(kitty_id).is_some(),true);

//         //check whether the event emited
//         System::assert_has_event(crate::Event::<Test>::KittyOnSale{who:account_id,kitty_id}.into());

//     })

// }

// #[test]
// fn it_works_for_buy(){
//     new_test_ext().execute_with(||{
//         let kitty_id=0;
//         let account_id_1=1;
//         let account_id_2=2;
//         let _=Balances::set_balance(RuntimeOrigin::root(), account_id_1, 10_000_000, 0);
//         let _=Balances::set_balance(RuntimeOrigin::root(), account_id_2, 10_000_000, 0);


//         //check the validation that whether the kitty_id is in Kitties 
//         assert_noop!(KittiesModule::buy(RuntimeOrigin::signed(account_id_2),kitty_id),Error::<Test>::InvalidKittyId);

//         //create a kitty by account_id and kitty_id
//         assert_ok!(KittiesModule::create(RuntimeOrigin::signed(account_id_1)));

//         //check whether the owner and the buyer is the same people
//         assert_noop!(KittiesModule::buy(RuntimeOrigin::signed(account_id_1),kitty_id),Error::<Test>::AlreadyOwned);

//         //sale the kitty
//         assert_ok!(KittiesModule::sale(RuntimeOrigin::signed(account_id_1),kitty_id));

//         //check whether the kitty is in KittyOnSale
//         assert_eq!(KittiesModule::kitty_on_sale(kitty_id).is_some(),true);

//         //buy the kitty
//         assert_ok!(KittiesModule::buy(RuntimeOrigin::signed(account_id_2),kitty_id));

//         //check the validation whether the kitty removed from KittyOnSale
//         assert_eq!(KittiesModule::kitty_on_sale(kitty_id).is_none(),true);

//         //check whether the owner is the account_id_2
//         assert_eq!(KittiesModule::kitty_owner(kitty_id),Some(account_id_2));


//         //check whether the event emited
//         System::assert_has_event(crate::Event::<Test>::KittyBought {who:account_id_2,kitty_id}.into());

//     })

// }