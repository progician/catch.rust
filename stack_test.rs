use stack::List;

test_case!( "Persistent list", {
    section!( "A new list", {
        let list = List::new();

        proposition!( "is empty", { require!( list.top() == None ); } );

        proposition!( "retains a single addition", { 
            let list = list.append( String::from( "Top" ) );
            require!( list.top() == Some( String::from( "Top" ) ) );
        } );

        proposition!( "retains additions in LIFO order then becomes empty", {
            let list = list.append( String::from( "A" ) );
            let list = list.append( String::from( "B" ) );
            let list = list.append( String::from( "C" ) );

            require!( list.top() == Some( String::from( "C" ) ) );

            let list = list.tail();
            require!( list.top() == Some( String::from( "B" ) ) );

            let list = list.tail();
            require!( list.top() == Some( String::from( "A" ) ) );

            let list = list.tail();
            require!( list.top() == None );
        } );

        proposition!( "will not iterate", {
            let mut list_iterator = list.iter();
            require!( list_iterator.next() == None );
        } );
    } );


    proposition!( "In an arbitrary list can count the elements and keep the list intact", {
        let list = List::new()
            .append( String::from( "A" ) )
            .append( String::from( "B" ) )
            .append( String::from( "C" ) );
        require!( list.top() == Some( String::from( "C" ) ) );
        require!( list.iter().count() == 3 );
        require!( list.top() == Some( String::from( "C" ) ) );
    } );
} );

