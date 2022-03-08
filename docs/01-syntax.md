# 文法について

後続のドキュメントで説明する機能は、すべて以下のような文法に基づいて `parser!` マクロの引数部分で設定されます。

```text
arg_kind = "pos_arg" | "arg_opt" | "opt" | "group" ;

modifier = IDENT , "=" , IDENT ;
modifiers = "(" , [ modifier , { "," , modifier } , [ "," ] ] , ")" ;

field_schema = IDENT , ":" , IDENT ;
schema_data_multi = "{" , [ field_schema , { "," , field_schema } , [ "," ] ] , "}" ;
schema_data = field_schema | schema_data_multi ;

schema = arg_kind , [ modifiers ] , schema_data ;

parser_macro_input = IDENT , { schema } ;
```
