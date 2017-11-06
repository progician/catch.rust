#[macro_export]
macro_rules! test_case {
    ( $test_case_title:expr, $test_case_body:block ) => {
        pub fn test_case_fn() -> bool {
            let test_case_title = $test_case_title;
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


            macro_rules! section {
                ( $section_title:expr, $section_body:block ) => {
                    {
                        let section_file = String::from( file!() );
                        let section_line = line!();
                        current_path.push_back( String::from( $section_title ) );
                        let mut having_checks = false;
                        let mut skip_checks = true;
                        let mut check_file = String::new();
                        let mut check_line = 0;

                        macro_rules! require {
                            ( $condition:expr ) => {
                                {
                                    check_line = line!();
                                    check_file = String::from( file!() );
                                    having_checks = true;
                                    if !collecting_tests {
                                        if !skip_checks {
                                            if !$condition {
                                                return false;
                                            }
                                        }
                                    }
                                }
                            };
                        }


                        if collecting_tests {
                            $section_body;
                            if having_checks {
                                leaf_proposals.push( current_path.clone() );
                            }
                        }
                        else if starts_with( &specific_proposal, &current_path ) {
                            if specific_proposal.len() == current_path.len() {
                                skip_checks = false;
                            }
                            let test_result = || -> bool {
                                $section_body
                                true
                            }();
                           
                            if !skip_checks {
                                println!( "-------------------------------------------------------------------------------" );
                                println!( "{}", test_case_title );
                                for section in current_path.clone() {
                                    println!( "  {}", section );
                                }
                                println!( "-------------------------------------------------------------------------------" );
                                println!( "{}:{}", section_file, section_line );
                                println!( "..............................................................................." );
                                println!();
                                println!( "{}:{}", check_file, check_line );
                                println!( "{}", if test_result { "PASSED" }
                                                else { "FAILED" } );
                                println!();
                            }
                        }
                        current_path.pop_back();
                    }
                }
            }


            $test_case_body;
            collecting_tests = false;
            for leaf_proposal in leaf_proposals.clone() {
                specific_proposal = leaf_proposal.clone();
                $test_case_body;
            }
            return true;
        }
    }
}

