// //! A simple parser for parsing rustache files.
// //!
// //! Can parse parse opening and closing rustaches and text nodes.

// use std::collections::hashmap::HashSet;
// use std::io::{File, BufferedReader};
use compiler::{Token, Text, Variable, OTag, CTag, Raw};
use std::mem;

#[deriving(Show, PartialEq, Eq, Clone)]
pub enum Node {
    Static(&'static str),
    Value(&'static str),
    Section(&'static str, Vec<Node>, bool), // (name, children, inverted)
    Unescaped(&'static str),
}

#[deriving(Show)]
pub struct Parser<'a> {
    tokens: &'a Vec<Token>,
    nodes: Vec<Node>
}

impl<'a> Parser<'a> {
    pub fn new<'a>(tokens: &'a Vec<Token>) -> Parser<'a> {
        let mut parser = Parser {
            tokens: tokens,
            nodes: vec![]
        };
        let mut nodes = parser.parse_nodes(parser.tokens);
        mem::swap(&mut parser.nodes, &mut nodes);
        parser
    }

    fn parse_nodes(&self, list: &'a Vec<Token>) -> Vec<Node> {
        let mut nodes: Vec<Node> = vec![];
        let len = list.len();
        for mut i in range(0, len) {
            match list[i] {
                Text(text)           => nodes.push(Static(text)),
                Variable(name)       => nodes.push(Value(name)),
                Raw(name)            => nodes.push(Unescaped(name)),
                OTag(name, inverted) => {
                    let mut children: Vec<Token> = vec![];
                    // iterate through the rest of the list (adding to children)
                    // until the closing tag is found
                    for j in range(i, len) {
                        match list[j] {
                            CTag(title) => {
                                if title == name {
                                    i = j;
                                    nodes.push(Section(name, self.parse_nodes(&children), inverted));
                                    break;
                                } else {
                                    children.push(list[j]);
                                    continue;
                                }
                            },
                            _           => {
                                children.push(list[j]);
                                continue;
                            }
                        }
                    }
                },
                CTag(name)           => {
                    // If this is triggered, its a closing tag that is present without
                    // an opening tag.  Representing bad syntax, error or ignore?
                    continue;
                },
            }
        }
        nodes
    }
}

//     pub fn read_template(template_path: &str) -> String {
//         let path = Path::new(template_path);

//         // Open the file path
//         let mut file = match File::open(&path) {
//             Err(why) => fail!("{}", why.desc),
//             Ok(file) => file,
//         };

//         // Read the file contents into a string
//         let contents = match file.read_to_string() {
//             Err(why) => fail!("{}", why.desc),
//             Ok(text) => text,
//         };

//         contents
//     }

//     pub fn create_map_from_tokens<'a>(nodes: Vec<Node>) -> HashSet<String> {
//         let mut tag_map: HashSet<String> = HashSet::new();
//         for node in nodes.iter() {
//             match *node {
//                 Value(ref text)  => tag_map.insert(text.clone()),
//                 Static(ref text) => continue,
//                 OTag(ref opt) => continue,
//                 CTag(ref opt) => continue,
//                 Inverted(ref text)  => continue,
//                 Unescaped(ref text)  => continue,
//             };        
//         }

//         tag_map
//     }
// }

#[test]
fn test_parser() {
    use compiler::Compiler;

    let contents = "Static String {{ token }} {{# open }}{{ tag }}{{/ open }}";
    let compiler = Compiler::new(contents);
    let parser = Parser::new(&compiler.tokens);
    for node in parser.nodes.iter() {
        println!("{}", node);
    }
    assert!(false);
}

// #[test]
// fn tokenize_should_map_strings() {
//     let test: &str = "Static tag!{{normal}}{{! comment }}!{{# tag }} {{/ tag }} {{^ inverted }} {{& unescaped }}";
//     let nodes = Parser::tokenize(test);
//     //should contain static blocks
//     assert_eq!(nodes.contains(&Static("Static tag!".to_string())), true);
//     //should not contain comment blocks
//     assert_eq!(nodes.contains(&Value("comment".to_string())), false);
//     //should contain open and close blocks
//     assert_eq!(nodes.contains(&OTag(Some("tag".to_string()))), true);
//     //should not contain unescaped blocks
//     assert_eq!(nodes.contains(&Unescaped("unescaped".to_string())), true);
// }

// #[test]
// fn mapper_should_create_a_set_of_useable_variables() {
//     let nodes = vec![Static("Static tag!".to_string()), Value("comment".to_string()), OTag(Some("tag".to_string()))];
//     let set = Parser::create_map_from_tokens(nodes);

//     // should only contain value nodes
//     assert!(set.contains(&"comment".to_string()));
// }
