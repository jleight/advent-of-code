namespace AoC.Collections;

public static partial class GridExtensions
{
    public static IEnumerable<Point<TMeta>> FindPathAStar<TMeta>(
        this Grid<double> grid,
        Point<TMeta> start,
        Point<TMeta> goal,
        Func<Point<TMeta>, Dictionary<Point<TMeta>, Point<TMeta>>, double> heuristic,
        Func<Point<TMeta>, Dictionary<Point<TMeta>, Point<TMeta>>, IEnumerable<Point<TMeta>>> getNeighbors)
    {
        var openSet = new HashSet<Point<TMeta>> { start };
        var cameFrom = new Dictionary<Point<TMeta>, Point<TMeta>>();
        var gScore = new Dictionary<Point<TMeta>, double> { [start] = 0 };
        var fScore = new Dictionary<Point<TMeta>, double> { [start] = heuristic(start, cameFrom) };

        while (openSet.Count > 0)
        {
            var current = openSet
                .OrderBy(p => fScore[p])
                .First();

            if (current == goal)
            {
                var path = new List<Point<TMeta>> { current };

                while (cameFrom.TryGetValue(current, out current))
                    path.Add(current);

                path.Reverse();
                return path;
            }

            openSet.Remove(current);

            foreach (var neighbor in getNeighbors(current, cameFrom))
            {
                var tgScore = gScore[current] + grid[neighbor.X, neighbor.Y];
                if (tgScore >= gScore.GetValueOrDefault(neighbor, double.PositiveInfinity))
                    continue;

                cameFrom[neighbor] = current;
                gScore[neighbor] = tgScore;
                fScore[neighbor] = tgScore + heuristic(neighbor, cameFrom);

                openSet.Add(neighbor);
            }
        }

        return new List<Point<TMeta>>();
    }
}
