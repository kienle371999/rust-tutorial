
mod my_mod {
    pub fn function() {
        println!("called `my_mod::function()`");
    }
}

mod my {
    #[allow(dead_code)]
    pub struct ClosedBox<T> {
        pub contents: T,
    }

    impl <T> ClosedBox<T> {
        pub fn new(contents: T) -> ClosedBox<T> {
            ClosedBox {
                contents: contents,
            }
        }
    }
}

mod cool {
    pub fn function() {
        println!("called `cool::function()`");
    }
}


mod another {
    fn function() {
        println!("called `my::function()`");
    }
    
    mod cool {
        pub fn function() {
            println!("called `another::cool::function()`");
        }
    }
    
    pub fn indirect_call() {
        // Let's access all the functions named `function` from this scope!
        print!("called `my::indirect_call()`, that\n> ");
        
        // The `self` keyword refers to the current module scope - in this case `my`.
        // Calling `self::function()` and calling `function()` directly both give
        // the same result, because they refer to the same function.
        self::function();
        function();
        
        // We can also use `self` to access another module inside `my`:
        self::cool::function();
        
        // The `super` keyword refers to the parent scope (outside the `my` module).
        super::cool::function();
        
        // This will bind to the `cool::function` in the *crate* scope.
        // In this case the crate scope is the outermost scope.
        {
            use crate::cool::function as root_function;
            root_function();
        }
    }
}

fn main () {
    my_mod::function();
    let _closed_box = my::ClosedBox::new("classified information");
    println!("The closed box contains: {}", _closed_box.contents);

    another::indirect_call();
}