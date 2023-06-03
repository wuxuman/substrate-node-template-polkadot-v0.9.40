use crate::{mock::*,Error};
use frame_support::{assert_ok,assert_noop};



#[test]
fn it_works_for_create(){
    new_test_ext().execute_with(||{
        let kitty_id=0;
        let account_id=1;

        assert_eq!(KittiesModule::next_kitty_id(),kitty_id);
        assert_ok!(KittiesModule::create(RuntimeOrigin::signed(account_id)));

        assert_eq!(KittiesModule::next_kitty_id(),kitty_id+1);
        assert_eq!(KittiesModule::kitties(kitty_id).is_some(),true);
        assert_eq!(KittiesModule::kitty_owner(kitty_id),Some(account_id));
        assert_eq!(KittiesModule::kitty_parents(kitty_id),None);

        crate::NextKittyId::<Test>::set(crate::KittyId::max_value());
        assert_noop!(
            KittiesModule::create(RuntimeOrigin::signed(account_id)),
            Error::<Test>::InvalidKittyId
        );

        // check whether create event emitted
        System::set_block_number(1);

        assert_eq!(System::events().len(), 1);

        let kitty=KittiesModule::kitties(kitty_id).unwrap();

        System::assert_has_event(crate::Event::<Test>::KittyCreated{who:account_id,kitty_id,kitty:kitty}.into());

    })

}

#[test]
fn it_works_for_breed(){
    new_test_ext().execute_with(||{
        let kitty_id=0;
        let account_id=1;

        assert_noop!(KittiesModule::breed(RuntimeOrigin::signed(1), kitty_id, kitty_id),
                    Error::<Test>::SameKittyId);

        assert_noop!(KittiesModule::breed(RuntimeOrigin::signed(1), kitty_id, kitty_id+1),
                    Error::<Test>::InvalidKittyId);     

        assert_ok!(KittiesModule::create(RuntimeOrigin::signed(account_id)));          
        assert_ok!(KittiesModule::create(RuntimeOrigin::signed(account_id))); 

        assert_eq!(KittiesModule::next_kitty_id(),kitty_id+2);

        assert_ok!(KittiesModule::breed(RuntimeOrigin::signed(account_id),kitty_id,kitty_id+1));      

        assert_eq!(KittiesModule::next_kitty_id(),kitty_id+3);

        assert_eq!(KittiesModule::kitties(2).is_some(),true);

        assert_eq!(KittiesModule::kitty_owner(2),Some(account_id));

        assert_eq!(KittiesModule::kitty_parents(2),Some((kitty_id,kitty_id+1)));


        // check whether create event emitted
        System::set_block_number(1);

        assert_eq!(System::events().len(), 3);

        let kitty=KittiesModule::kitties(kitty_id+2).unwrap();

        System::assert_has_event(crate::Event::<Test>::KittyCreated{who:account_id,kitty_id,kitty:kitty}.into());
        System::assert_has_event(crate::Event::<Test>::KittyCreated{who:account_id,kitty_id:kitty_id+1,kitty:kitty}.into());
        System::assert_has_event(crate::Event::<Test>::KittyBreed {who:account_id,kitty_id:kitty_id+2,kitty:kitty}.into());


    });
}



#[test]
fn it_works_for_transfer(){
    new_test_ext().execute_with(||{

        let kitty_id=0;
        let account_id=1;
        let recipient_id=2;

        assert_ok!(KittiesModule::create(RuntimeOrigin::signed(account_id)));

        assert_noop!(KittiesModule::transfer(RuntimeOrigin::signed(10), recipient_id, kitty_id),
                    Error::<Test>::NotOwner);

        assert_ok!(KittiesModule::transfer(RuntimeOrigin::signed(account_id), recipient_id, kitty_id));

        assert_eq!(KittiesModule::kitty_owner(kitty_id),Some(recipient_id));

        assert_ok!(KittiesModule::transfer(RuntimeOrigin::signed(recipient_id), account_id, kitty_id));

        assert_eq!(KittiesModule::kitty_owner(kitty_id),Some(account_id));

      // check whether create event emitted
      System::set_block_number(1);

      assert_eq!(System::events().len(), 3);

      let kitty=KittiesModule::kitties(kitty_id).unwrap();

      System::assert_has_event(crate::Event::<Test>::KittyCreated{who:account_id,kitty_id,kitty:kitty}.into());
      System::assert_has_event(crate::Event::<Test>::KittyTransferred {who:account_id,recipient: recipient_id,kitty_id:kitty_id}.into());
      System::assert_has_event(crate::Event::<Test>::KittyTransferred {who:recipient_id,recipient: account_id,kitty_id:kitty_id}.into());

    });
}