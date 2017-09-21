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


fn a_new_list_is_empty() {
    let list = List::new();
    assert!( list.top() == None );
}


fn an_empty_list_retains_a_single_addition() {
    let list = List::new().append( String::from( "Top" ) );
    assert!( list.top() == Some( String::from( "Top" ) ) );
}

fn an_empty_list_retains_additions_in_lifo_order_then_becomes_empty() {
    let list = List::new();
    let list = list.append( String::from( "A" ) );
    let list = list.append( String::from( "B" ) );
    let list = list.append( String::from( "C" ) );

    assert_eq!( list.top(), Some( String::from( "C" ) ) );

    let list = list.tail();
    assert!( list.top() == Some( String::from( "B" ) ) );

    let list = list.tail();
    assert!( list.top() == Some( String::from( "A" ) ) );

    let list = list.tail();
    assert!( list.top() == None );
}


fn an_empty_list_will_not_iterate() {
    let list = List::new();
    let mut list_iterator = list.iter();
    assert!( list_iterator.next() == None );
}


fn iterators_for_an_arbitrary_list_read_the_elements_will_keep_the_list_intact() {
    let list = List::new()
        .append( String::from( "A" ) )
        .append( String::from( "B" ) )
        .append( String::from( "C" ) );
    assert!( list.top() == Some( String::from( "C" ) ) );

    assert!( list.iter().count() == 3 );
    assert!( list.top() == Some( String::from( "C" ) ) );
}


use std::collections::HashMap;

fn main() {
    a_new_list_is_empty();
    an_empty_list_retains_a_single_addition();
    an_empty_list_retains_additions_in_lifo_order_then_becomes_empty();
    an_empty_list_will_not_iterate();
    iterators_for_an_arbitrary_list_read_the_elements_will_keep_the_list_intact();

    println!( "All tests has passed" );
}