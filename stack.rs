struct List {
    head: Link,
}

use std::rc::Rc;
type Link = Option< Rc< Node > >;

struct Node {
    value: String,
    rest: Link,
}

struct ListIter< 'a > {
    next: Option<&'a Node>,
}

impl List {
    pub fn new() -> Self {
        List { head: None }
    }


    pub fn append( &self, new_value: String ) -> Self {
        List { head: Some( Rc::new( Node {
            value: new_value,
            rest: self.head.clone(),
        })) }
    }


    pub fn tail( &self ) -> Self {
        List {
            head: self.head.as_ref().
                and_then( |node| node.rest.clone() )
        }
    }


    pub fn top( &self ) -> Option<String> {
        self.head.as_ref().map( |node| { node.value.clone() } )
    }


    pub fn iter< 'a >( &'a self ) -> ListIter< 'a > {
        ListIter {
            next: self.head.
                as_ref().map( |node| &**node )
        }
    }
}


impl< 'a > Iterator for ListIter< 'a > {
    type Item = &'a String;

    fn next( &mut self ) -> Option< Self::Item > {
        self.next.map( |node| {
            self.next = node.rest.as_ref().map( |node| &**node );
            &node.value
        } )
    }
}


macro_rules! require {
    ( $x:expr ) => {
        {
            if !$x {
                return false 
            }
        }
    };
}


macro_rules! test_case {
    ( $x:expr, $proposals:block ) => {
        fn main() {
            use std::collections::LinkedList;
            type TestPath = LinkedList< String >;
            let mut sections: LinkedList< TestPath > = LinkedList::new();
            let mut current_path: LinkedList< String > = LinkedList::new();
            let mut dry_run = true;

            macro_rules! proposition {
                ( $title:expr, $body:block ) => {
                    {
                        current_path.push_back( String::from( $title ) );

                        if dry_run {
                            sections.push_back( current_path.clone() );
                            || -> bool { 
                                $body
                                true
                            }();
                        }
                        else {
                            let mut output_str = String::new();
                            for _ in 0..current_path.len() - 1 {
                                output_str.push_str( "  " );
                            }

                            output_str.push_str( $title );
                            output_str.push_str( ": " );
                            println!( "{}", output_str );

                            let test_result = || -> bool {
                                $body
                                true
                            }();

                            println!( "{}", match test_result {
                                true => "PASSED",
                                false => "FAILED!",
                            } );
                        }
                        current_path.pop_back();
                    }
                }
            }

            for _ in 0..2 {
                $proposals
                dry_run = false;
            }
            
            // println!( "All propositions has passed" );
        }
    }
}


test_case!( "Persistent list", {
    proposition!( "An new list", {
        let list = List::new();
        require!( list.top() == None );

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


