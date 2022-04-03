let PosArg =
    { Type =
        { rust_type: Text
        , possible_values: Optional (List Text)
        , default_value: Optional Text
        }
    , default =
        { possible_values = None (List Text)
        , default_value = None Text
        }
    }

let Flag =
    < Both: { long: Text, short: Text }
    | LongOnly: Text
    | ShortOnly: Text
    >

let Opt =
    { rust_type: Text
    , flag: Flag
    }

let ArgOpt =
    { rust_type: Text
    , flag: Flag
    }

let Member =
    < PosArg: PosArg.Type
    | Opt: Opt
    | ArgOpt: ArgOpt
    >

let sourceFileArg = PosArg::{ rust_type = "SourceFile" }

let outfileOpt: ArgOpt = {
    rust_type = "Outfile", -- generated Rust type
    flag = Flag.Both { long = "outfile", short = "o" }
}

in

[Member.PosArg sourceFileArg, Member.ArgOpt outfileOpt]: List Member
