pub mod graph {
    use std::collections::HashMap;

    #[derive(Default)]
    pub struct Graph<'a> {
        pub nodes: Vec<graph_items::node::Node<'a>>,
        pub edges: Vec<graph_items::edge::Edge<'a>>,
        pub attrs: HashMap<&'a str, &'a str>,
    }

    impl<'a> Graph<'a> {
        pub fn new() -> Self {
            Default::default()
        }

        pub fn with_nodes<'b>(mut self, nodes: &'b [graph_items::node::Node<'a>]) -> Self {
            self.nodes = nodes.to_vec();
            self
        }

        pub fn with_edges<'b>(mut self, edges: &'b [graph_items::edge::Edge<'a>]) -> Self {
            self.edges = edges.to_vec();
            self
        }

        pub fn with_attrs(mut self, attrs: &[(&'a str, &'a str)]) -> Self {
            self.attrs.extend(attrs.iter().copied());
            self
        }

        pub fn node(&'a self, label: &'a str) -> Option<&'a graph_items::node::Node<'a>> {
            self.nodes.iter().find(|&node| node.label() == label)
        }
    }

    pub mod graph_items {
        pub mod edge {
            use super::super::HashMap;

            #[derive(Clone, Debug, Default, PartialEq)]
            pub struct Edge<'a> {
                from: &'a str,
                to: &'a str,
                attrs: HashMap<&'a str, &'a str>,
            }

            impl<'a> Edge<'a> {
                pub fn new(from: &'a str, to: &'a str) -> Self {
                    Self {
                        from,
                        to,
                        ..Default::default()
                    }
                }

                pub fn with_attrs(mut self, attrs: &[(&'a str, &'a str)]) -> Self {
                    self.attrs.extend(attrs.iter().copied());
                    self
                }

                pub fn attr(&self, name: &'a str) -> Option<&'a str> {
                    self.attrs.get(name).copied()
                }
            }
        }

        pub mod node {
            use super::super::HashMap;

            #[derive(Clone, Debug, PartialEq, Default)]
            pub struct Node<'a> {
                label: &'a str,
                attrs: HashMap<&'a str, &'a str>,
            }

            impl<'a> Node<'a> {
                pub fn new(label: &'a str) -> Self {
                    Self {
                        label,
                        ..Default::default()
                    }
                }

                pub fn label(&self) -> &'a str {
                    self.label
                }

                pub fn with_attrs(mut self, attrs: &[(&'a str, &'a str)]) -> Self {
                    self.attrs.extend(attrs.iter().copied());
                    self
                }

                pub fn attr(&self, name: &'a str) -> Option<&'a str> {
                    self.attrs.get(name).copied()
                }
            }
        }
    }
}
