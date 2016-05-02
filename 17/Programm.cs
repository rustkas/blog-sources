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

        /// <summary>
        /// Constructor.
        /// </summary>
        /// <param name="value">Value.</param>
        /// <param name="owner">Owning graph.</param>
        public Node(string value, Graph owner)
        {
            Value = value;
            this.owner = owner;
        }

        /// <summary>
        /// Return list of nodes connected with this node.
        /// </summary>
        /// <returns>List of connected nodes.</returns>
        public Dictionary<Node, Weight> GetConnectedNodes()
        {
            return owner.GetConnetedNodes(this);
        }
    }

    class Graph
    {
        Dictionary<Node, Dictionary<Node, Weight>> map = new Dictionary<Node, Dictionary<Node, Weight>>();
        Weight defaultWeight = 1;

        /// <summary>
        /// Add to graph new node with the specified value.
        /// </summary>
        /// <param name="value">Added value.</param>
        /// <returns>Created node.</returns>
        public Node CreateNode(string value)
        {
            var node = new Node(value, this);
            map.Add(node, new Dictionary<Node, Weight>());
            return node;
        }

        /// <summary>
        /// Add edge between two nodes.
        /// </summary>
        /// <param name="n1">Begin node.</param>
        /// <param name="n2">End node.</param>
        /// <param name="bidirect">Create bidirect edge.</param>
        public void AddEdge(Node n1, Node n2, bool bidirect)
        {
            AddEdge(n1, n2, bidirect, defaultWeight);
        }

        /// <summary>
        /// Add edge between two nodes.
        /// </summary>
        /// <param name="n1">Begin node.</param>
        /// <param name="n2">End node.</param>
        /// <param name="bidirect">Create bidirect edge.</param>
        /// <param name="weight">Weight of edge.</param>
        public void AddEdge(Node n1, Node n2, bool bidirect, Weight weight)
        {
            map[n1].Add(n2, weight);
            if (bidirect)
            {
                map[n2].Add(n1, weight);
            }
        }

        /// <summary>
        /// Return list of nodes connected with specified node.
        /// </summary>
        /// <param name="node">List of connected nodes.</param>
        /// <returns></returns>
        public Dictionary<Node, Weight> GetConnetedNodes(Node node)
        {
            return map.ContainsKey(node) ? map[node] : new Dictionary<Node, int>();
        }
    }
}
