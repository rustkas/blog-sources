using System;
using System.Collections.Generic;

namespace ConsoleApplication
{
    class Program
    {
        static void Main(string[] args)
        {
            var n1 = new Node("Node 1");
            var n2 = new Node("Node 2");
            var n3 = new Node("Node 3");
            var n4 = new Node("Node 4");
            var n5 = new Node("Node 5");
            var n6 = new Node("Node 6");
            var n7 = new Node("Node 7");
            n1.AddNodes(new Node[] { n2, n3 });
            n2.AddNode(n3);
            n3.AddNodes(new Node[] { n1, n4, n5, n7 });
            n4.AddNodes(new Node[] { n3, n6 });
            n5.AddNode(n4);
            n6.AddNode(n1);

            var dfs = new DFS();
            DFS.Callback callback = (node) => Console.WriteLine(node.Name);
            Console.WriteLine("Recursive");
            dfs.Search(n1, callback);
            Console.WriteLine("");
            Console.WriteLine("Not recursive");
            dfs.SearchRecursive(n1, callback);
            Console.Read();
        }
    }

    /// <summary>
    /// Depth-first search
    /// </summary>
    class DFS
    {
        public delegate void Callback(Node node);

        private HashSet<Node> visited;

        /// <summary>
        /// Recursively explore all nodes started at `start`.
        /// </summary>
        /// <param name="start">Start node</param>
        /// <param name="callback">Callback.</param>
        public void SearchRecursive(Node start, Callback callback)
        {
            visited = new HashSet<Node>();
            searchRecursive(start, callback);
        }

        private void searchRecursive(Node current, Callback callback)
        {
            visited.Add(current);
            callback(current);
            foreach (var node in current.Nodes)
            {
                if (!visited.Contains(node))
                {
                    searchRecursive(node, callback);
                }
            }
        }

        /// <summary>
        /// Not recursively explore all nodes started at `start`.
        /// </summary>
        /// <param name="start">Start node</param>
        /// <param name="callback">Callback.</param>
        public void Search(Node start, Callback callback)
        {
            var visited = new HashSet<Node>() { start };
            var stack = new Stack<Node>();
            stack.Push(start);

            while (stack.Count > 0)
            {
                var current = stack.Pop();
                visited.Add(current);
                callback(current);
                foreach (var node in current.Nodes)
                {
                    if (!visited.Contains(node))
                    {
                        stack.Push(node);
                        visited.Add(node);
                    }
                }
            }

        }

    }

    class Node
    {
        public string Name { get; }
        public List<Node> Nodes { get; }

        public Node(string name)
        {
            Name = name;
            Nodes = new List<Node>();
        }

        public void AddNode(Node node)
        {
            Nodes.Add(node);
        }

        public void AddNodes(Node[] nodes)
        {
            foreach (var node in nodes)
            {
                AddNode(node);
            }
        }
    }
}
