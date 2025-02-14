// #![feature(proc_macro_diagnostic)]
// #![feature(iter_array_chunks)]

extern crate proc_macro;
extern crate syn;
#[macro_use]
extern crate quote;
use proc_macro::{TokenStream};
use syn::{parse_macro_input, DeriveInput};


#[proc_macro_derive(RoleTable)]
pub fn role_table_derive(input: TokenStream) -> TokenStream {
    // 解析输入的 TokenStream 为一个 `DeriveInput` 语法树节点
    let input = parse_macro_input!(input as DeriveInput);
    // Diagnostic::new(Level::Warning, "This is a warning message").emit();
    // 获取类型的名称
    let name = &input.ident;
    dbg!(&input); // 这个调试信息将在编译器输出中出现
    eprintln!("Debug info: {:?}", input); // 这个信息会输出到标准错误输出

    let s = match input.data {
        syn::Data::Struct(s) => {Ok(s)}
        _=>{Err(())}
    }.unwrap();
    let mut load_fn1 = quote!{select};
    s.fields.iter().for_each(|it|{
        let field_ident = it.ident.clone().unwrap();
        load_fn1 = quote!{
            #load_fn1 #field_ident,
        };
    });
    load_fn1 = quote!{
        #load_fn1 from account
    };

    // 使用 `quote!` 宏生成实现代码
    let expanded = quote! {
        impl RoleTable for #name {
            fn load(conn:&mut mysql::PooledConn, user_id: i64) -> Result<Option<Self>, Box<dyn std::error::Error>> {
                let row:Option<Account> = conn.query_first(stringify!(#load_fn1))?;
                Ok(row)
            }
        }
    };
    // info!("Info level log: {:?}", input);
    // warn!("Warning level log: {:?}", input);
    // 将生成的代码转换为 TokenStream 并返回
    TokenStream::from(expanded)
}

