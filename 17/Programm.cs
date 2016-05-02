using System;
using System.Collections.Generic;
using Weight = System.Int32;

namespace ConsoleApplication
{
    class Program
    {
        static void Main(string[] args)
        {
            var n1 = new Node("node 1");
            var n2 = new Node("node 2");
            var n3 = new Node("node 3");
            var n4 = new Node("node 4");
            var n5 = new Node("node 5");
            var n6 = new Node("node 6");
            var n7 = new Node("node 7");
            n1.Add(n2, 3);
            n1.Add(n3, 2);
            n2.Add(n3, 2);
            n3.Add(n7, 6);
            n3.Add(n5, 1);
            n3.Add(n4, 9);
            n3.Add(n1, 2);
            n4.Add(n3, 9);
            n5.Add(n4, 2);
            n6.Add(n1, 4);
        }
    }

    class Node
    {
        public string Value { get; }
        public Dictionary<Node, Weight> Children { get; }

        public Node(string value)
        {
            Value = value;
            Children = new Dictionary<Node, Weight>();
        }

        public Node(string value, Dictionary<Node, Weight> children)
        {
            Value = value;
            Children = children;
        }

        public void Add(Node node, Weight weight)
        {
            Children.Add(node, weight);
        }
    }
}
