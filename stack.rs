pub struct List {
    head: Link,
}

use std::rc::Rc;
type Link = Option< Rc< Node > >;

struct Node {
    value: String,
    rest: Link,
}

pub struct ListIter< 'a > {
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


