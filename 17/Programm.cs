using System;
using System.Collections.Generic;
using Weight = System.Int32;

namespace ConsoleApplication
{
    class Program
    {
        static void Main(string[] args)
        {

        }
    }


    class Node
    {
        public string Value { get; }
        Graph owner;

        public Node(string value, Graph owner)
        {
            Value = value;
            this.owner = owner;
        }

        public Dictionary<Node, Weight> GetConnectedNodes()
        {
            return owner.GetConnetedNodes(this);
        }
    }

    class Graph
    {
        Dictionary<Node, Dictionary<Node, Weight>> map = new Dictionary<Node, Dictionary<Node, Weight>>();

        public Node CreateNode(string value)
        {
            var node = new Node(value, this);
            map.Add(node, new Dictionary<Node, Weight>());
            return node;
        }

        public void AddEdge(Node n1, Node n2, Weight weight)
        {
            map[n1].Add(n2, weight);
        }

        public void AddEdge(Node n1, Node n2, Weight weight, bool bidirect)
        {
            AddEdge(n1, n2, weight);
            if (bidirect)
            {
                AddEdge(n2, n1, weight);
            }
        }

        public Dictionary<Node, Weight> GetConnetedNodes(Node node)
        {
            return map.ContainsKey(node) ? map[node] : new Dictionary<Node, int>();
        }
    }
}
