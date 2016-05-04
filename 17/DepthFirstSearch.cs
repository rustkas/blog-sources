using System;
using System.Collections.Generic;
using System.Linq;

namespace Graphs
{
    class DepthFirstSearch
    {
        private HashSet<Node> visited;
        private LinkedList<Node> path;
        private Node goal;

        public LinkedList<Node> DFS(Node start, Node goal)
        {
            Console.WriteLine($"Search path from {start.Name} to {goal.Name}:");
            visited = new HashSet<Node>();
            path = new LinkedList<Node>();
            this.goal = goal;
            DFS(start);
            if (path.Count > 0)
            {
                path.AddFirst(start);
            }
            return path;
        }

        private bool DFS(Node node)
        {
            node.Handler();
            if (node == goal)
            {
                return true;
            }
            visited.Add(node);
            foreach (var child in node.Children.Where(x => !visited.Contains(x)))
            {
                if (DFS(child))
                {
                    path.AddFirst(child);
                    return true;
                }
            }
            return false;
        }

        public LinkedList<Node> DLS(Node start, Node goal, int limit)
        {
            Console.WriteLine($"Search path from {start.Name} to {goal.Name}:");
            visited = new HashSet<Node>();
            path = new LinkedList<Node>();
            this.goal = goal;
            DLS(start, limit);
            if (path.Count > 0)
            {
                path.AddFirst(start);
            }
            return path;
        }

        private bool DLS(Node node, int limit)
        {
            node.Handler();
            if (node == goal)
            {
                return true;
            }
            if (limit == 0)
            {
                return false;
            }
            visited.Add(node);
            foreach (var child in node.Children.Where(x => !visited.Contains(x)))
            {
                if (DLS(child, limit-1))
                {
                    path.AddFirst(child);
                    return true;
                }
            }
            return false;
        }
    }
}
