#[doc = "

Pulls a brief description out of a long description.

If the first paragraph of a long description is short enough then it
is interpreted as the brief description.

"];

export mk_pass;

fn mk_pass() -> pass {
    {
        name: "desc_to_brief",
        f: run
    }
}

fn run(
    _srv: astsrv::srv,
    doc: doc::doc
) -> doc::doc {
    let fold = fold::fold({
        fold_item: fold_item,
        fold_iface: fold_iface,
        fold_impl: fold_impl
        with *fold::default_any_fold(())
    });
    fold.fold_doc(fold, doc)
}

fn fold_item(fold: fold::fold<()>, doc: doc::itemdoc) -> doc::itemdoc {
    let doc = fold::default_seq_fold_item(fold, doc);

    {
        brief: extract(doc.desc)
        with doc
    }
}

fn fold_iface(fold: fold::fold<()>, doc: doc::ifacedoc) -> doc::ifacedoc {
    let doc =fold::default_seq_fold_iface(fold, doc);

    {
        methods: par::anymap(doc.methods) {|doc|
            {
                brief: extract(doc.desc)
                with doc
            }
        }
        with doc
    }
}

fn fold_impl(fold: fold::fold<()>, doc: doc::impldoc) -> doc::impldoc {
    let doc =fold::default_seq_fold_impl(fold, doc);

    {
        methods: par::anymap(doc.methods) {|doc|
            {
                brief: extract(doc.desc)
                with doc
            }
        }
        with doc
    }
}

#[test]
fn should_promote_desc() {
    let doc = test::mk_doc("#[doc = \"desc\"] mod m { }");
    assert doc.cratemod().mods()[0].brief() == some("desc");
}

#[test]
fn should_promote_iface_method_desc() {
    let doc = test::mk_doc("iface i { #[doc = \"desc\"] fn a(); }");
    assert doc.cratemod().ifaces()[0].methods[0].brief == some("desc");
}

#[test]
fn should_promote_impl_method_desc() {
    let doc = test::mk_doc(
        "impl i for int { #[doc = \"desc\"] fn a() { } }");
    assert doc.cratemod().impls()[0].methods[0].brief == some("desc");
}

#[cfg(test)]
mod test {
    fn mk_doc(source: str) -> doc::doc {
        astsrv::from_str(source) {|srv|
            let doc = extract::from_srv(srv, "");
            let doc = attr_pass::mk_pass().f(srv, doc);
            run(srv, doc)
        }
    }
}

fn extract(desc: option<str>) -> option<str> {
    if option::is_none(desc) {
        ret none
    }

    parse_desc(option::get(desc))
}

fn parse_desc(desc: str) -> option<str> {

    const max_brief_len: uint = 120u;

    alt first_sentence(desc) {
      some(first_sentence) {
        if str::len(first_sentence) <= max_brief_len {
            some(first_sentence)
        } else {
            none
        }
      }
      none { none }
    }
}

fn first_sentence(s: str) -> option<str> {
    let paras = paragraphs(s);
    if vec::is_not_empty(paras) {
        let first = vec::head(sentences(vec::head(paras)));
        some(str::replace(first, "\n", " "))
    } else {
        none
    }
}

fn sentences(s: str) -> [str] {
    str::split_char(s, '.')
}

fn paragraphs(s: str) -> [str] {
    let lines = str::lines_any(s);
    let whitespace_lines = 0;
    let accum = "";
    let paras = vec::foldl([], lines) {|paras, line|
        let res = paras;

        if str::is_whitespace(line) {
            whitespace_lines += 1;
        } else {
            if whitespace_lines > 0 {
                if str::is_not_empty(accum) {
                    res += [accum];
                    accum = "";
                }
            }

            whitespace_lines = 0;

            accum = if str::is_empty(accum) {
                line
            } else {
                accum + "\n" + line
            }
        }

        res
    };

    if str::is_not_empty(accum) {
        paras + [accum]
    } else {
        paras
    }
}

#[test]
fn test_paragraphs_1() {
    let paras = paragraphs("1\n\n2");
    assert paras == ["1", "2"];
}

#[test]
fn test_paragraphs_2() {
    let paras = paragraphs("\n\n1\n1\n\n2\n\n");
    assert paras == ["1\n1", "2"];
}

#[test]
fn should_promote_short_descs() {
    let desc = some("desc");
    let brief = extract(desc);
    assert brief == desc;
}

#[test]
fn should_not_promote_long_descs() {
    let desc = some("Warkworth Castle is a ruined medieval building
in the town of the same name in the English county of Northumberland,
and the town and castle occupy a loop of the River Coquet, less than a mile
from England's north-east coast. When the castle was founded is uncertain,
but traditionally its construction has been ascribed to Prince Henry of
Scotland in the mid 12th century, although it may have been built by
King Henry II of England when he took control of England'snorthern
counties.");
    let brief = extract(desc);
    assert brief == none;
}

#[test]
fn should_promote_first_sentence() {
    let desc = some("Warkworth Castle is a ruined medieval building
in the town. of the same name in the English county of Northumberland,
and the town and castle occupy a loop of the River Coquet, less than a mile
from England's north-east coast. When the castle was founded is uncertain,
but traditionally its construction has been ascribed to Prince Henry of
Scotland in the mid 12th century, although it may have been built by
King Henry II of England when he took control of England'snorthern
counties.");
    let brief = extract(desc);
    assert brief == some(
        "Warkworth Castle is a ruined medieval building in the town");
}