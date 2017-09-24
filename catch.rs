


macro_rules! require {
    ( $x:expr ) => {
        {
            if !$x {
                return false;
            }
        }
    };
}


#[macro_export]
macro_rules! test_case {
    ( $x:expr, $proposals:block ) => {
        pub fn test_case_fn() -> bool {
            let mut collecting_tests = true;

            let mut leaf_proposals = Vec::new();

            use std::collections::LinkedList;
            type StringList = LinkedList< String >;
            let mut current_path = StringList::new();
            let mut specific_proposal = StringList::new();

            fn starts_with( full: &StringList, starting_sequence: &StringList ) -> bool {
                if full.len() < starting_sequence.len() {
                    return false;
                }
                
                let mut full_it = full.iter();
                
                for starting_value in starting_sequence.iter() {
                    let full_value = full_it.next();
                    match full_value {
                        None => return false,
                        Some( full_value ) => {
                            if *full_value != *starting_value {
                                return false;
                            }
                        }
                    }
                }
                return true;
            }

            macro_rules! proposition {
                ( $title:expr, $body:block ) => {
                    {
                        current_path.push_back( String::from( $title ) );
                        if collecting_tests {
                            leaf_proposals.push( current_path.clone() );
                        }
                        else if starts_with( &specific_proposal, &current_path ){
                            let test_result = || -> bool {
                                $body
                                true
                            }();
                            
                            for section in current_path.clone() {
                                println!( "{}", section );
                            }
                            println!( "{}", if test_result { "PASSED" }
                                            else { "FAILED" } );
                        }
                        current_path.pop_back();
                    }
                }
            }

            macro_rules! section {
                ( $title:expr, $body:block ) => {
                    {
                        current_path.push_back( String::from( $title ) );
                        if collecting_tests {
                            || -> bool {
                                $body
                                true
                            }();
                        }
                        else if starts_with( &specific_proposal, &current_path ){
                            || -> bool {
                                $body
                                true
                            }();
                        }
                        current_path.pop_back();
                    }
                }
            }
            

            $proposals;
            collecting_tests = false;
            for leaf_proposal in leaf_proposals.clone() {
                specific_proposal = leaf_proposal.clone();
                $proposals;
            }
            return true;
        }
    }
}

