//! Simple router implementation inspired by Rouille's router! macro.

#[macro_export]
macro_rules! route {
    ( $route_name: ident: $method: ident ( $($pattern: tt)+ ) ) => {
        charon::route!( @struct $route_name ($( $pattern )+) ());

        impl charon::Route for $route_name {
            fn check(method: &hyper::Method, components: &[&str]) -> Option<Self> {
                if method != &hyper::Method::$method {
                    return None;
                }

                $(
                    charon::route!( @chomp components $pattern);
                )+

                if !components.is_empty() {
                    return None;
                }

                Some(charon::route!( @initializer ($($pattern)+) ()))
            }
        }
    };

    // terminal rule
    ( @struct $route_name: ident () ( $( $emitted_so_far: tt )* ) ) => {
        struct $route_name {
            $( $emitted_so_far )*
        }
    };

    // Slashes don't impact the struct
    ( @struct $route_name: ident (/ $($pattern: tt)*) ($($emitted_so_far: tt)*) ) => {
        charon::route!( @struct $route_name ($($pattern)*) ($($emitted_so_far)*) );
    };

    // Stars don't impact the struct
    ( @struct $route_name: ident (* $($pattern: tt)*) ($($emitted_so_far: tt)*) ) => {
        charon::route!( @struct $route_name ($($pattern)*) ($($emitted_so_far)*) );
    };

    // Plain path components don't impact the struct
    ( @struct $route_name: ident ($component: ident $($pattern: tt)*) ($($emitted_so_far: tt)*) ) => {
        charon::route!( @struct $route_name ($($pattern)*) ($($emitted_so_far)*) );
    };

    // Variable path components turn into members on the struct
    ( @struct $route_name: ident ({ $member: ident: $ty: ty } $($pattern: tt)*) ($($emitted_so_far: tt)*) ) => {
        charon::route!( @struct $route_name ($($pattern)*) ( $member: $ty, $($emitted_so_far)*) );
    };

    // all other invalid path components
    ( @struct $route_name: ident ($($pattern: tt)+) ($($emitted_so_far: tt)*) ) => {
        compile_error!("Unexpected path component");
    };

    // terminal rule
    ( @initializer () ( $( $emitted_so_far: tt )* ) ) => {
        Self {
            $( $emitted_so_far )*
        }
    };

    // a slash
    ( @initializer (/ $($pattern: tt)*) ($($emitted_so_far: tt)*) ) => {
        // no emit in struct definition

        charon::route!( @initializer ($($pattern)*) ($($emitted_so_far)*) );
    };

    // a star
    ( @initializer (* $($pattern: tt)*) ($($emitted_so_far: tt)*) ) => {
        // no emit in struct definition

        charon::route!( @initializer ($($pattern)*) ($($emitted_so_far)*) );
    };

    // plain path component
    ( @initializer ($component: ident $($pattern: tt)*) ($($emitted_so_far: tt)*) ) => {
        // no emit in struct definition

        charon::route!( @initializer ($($pattern)*) ($($emitted_so_far)*) );
    };

    // path component with variable
    ( @initializer ({ $member: ident: $ty: ty } $($pattern: tt)*) ($($emitted_so_far: tt)*) ) => {
        charon::route!( @initializer ($($pattern)*) ( $member, $($emitted_so_far)*) );
    };

    // all other invalid path pieces
    ( @initializer ($($pattern: tt)+) ($($emitted_so_far: tt)*) ) => {
        compile_error!("Unexpected path component");
    };

    ( @chomp $components: ident / ) => {};
    ( @chomp $components: ident * ) => {
        let $components = charon::internals::chomp_any($components)?;
    };
    ( @chomp $components: ident $component: ident) => {
        let $components = charon::internals::chomp_exact($components, stringify!($component))?;
    };
    ( @chomp $components: ident { $member: ident: $ty: ty }) => {
        let ($components, $member) = charon::internals::chomp_ty::<$ty>($components)?;
    };
}

#[macro_export]
macro_rules! router {
    ( $router_name: ident {$(
        $route_name: ident: $method: ident ( $( $pattern: tt )+ ),
    )*}) => {
        $(
            charon::route!($route_name: $method ( $( $pattern )+ ));
        )*

        enum $router_name {
            $(
                $route_name($route_name),
            )*
        }

        impl $router_name {
            fn route(method: &hyper::Method, uri: &str) -> Option<Self> {
                use charon::Route;

                let components: Vec<_> = uri.split('/').collect();

                $(
                    if let Some(route) = $route_name::check(method, &components) {
                        return Some(Self::$route_name(route))
                    }
                )*

                None
            }
        }
    };
}
