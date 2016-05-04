using System;
using System.Collections.Generic;

namespace Graphs
{
    class Program
    {
        static void Main(string[] args)
        {
            var n01 = new Node("01");
            var n02 = new Node("02");
            var n03 = new Node("03");
            var n04 = new Node("04");
            var n05 = new Node("05");
            var n06 = new Node("06");
            var n07 = new Node("07");
            var n08 = new Node("08");
            var n09 = new Node("09");
            var n10 = new Node("10");
            var n11 = new Node("11");
            var n12 = new Node("12");
            var n13 = new Node("13");
            var n14 = new Node("14");
            var n15 = new Node("15");
            n01.AddChildren(n02).AddChildren(n03);
            n02.AddChildren(n05);
            n03.AddChildren(n04);
            n04.AddChildren(n05, false).AddChildren(n10, false).AddChildren(n11, false);
            n06.AddChildren(n01, false);
            n07.AddChildren(n03, false).AddChildren(n08);
            n09.AddChildren(n08).AddChildren(n10);
            n11.AddChildren(n12).AddChildren(n13);
            n12.AddChildren(n13);
            n14.AddChildren(n15);

            var search = new DepthFirstSearch();
            var path = search.DLS(n06, n13,6);
            PrintPath(path);
        }

        private static void PrintPath(LinkedList<Node> path)
        {
            Console.WriteLine();
            if (path.Count==0)
            {
                Console.WriteLine("You shall not pass!");
            }
            else
            {
                Console.WriteLine("Path:");
                foreach (var node in path)
                {
                    Console.WriteLine(node.Name);
                }
            }
            Console.Read();
        }
    }

    class Node
    {
        public string Name { get; }
        public List<Node> Children { get; }

        public Node(string name)
        {
            Name = name;
            Children = new List<Node>();
        }

        public Node AddChildren(Node node, bool bidirect = true)
        {
            Children.Add(node);
            if (bidirect)
            {
                node.Children.Add(this);
            }
            return this;
        }

        public void Handler()
        {
            Console.WriteLine($"visited {this.Name}");
        }
    }
}
