namespace AoC.Y2023.Solutions;

public class D13P2 : ISolution
{
    public Task Run(SolutionContext context)
    {
        var sum = context.InputLines
            .ChunkBy(string.IsNullOrWhiteSpace)
            .Select(c => c.Select(l => l.ToCharArray()).ToArray())
            .Select(CalculateSolution)
            .Sum();

        Console.WriteLine(sum);
        return Task.CompletedTask;
    }

    private static int CalculateSolution(char[][] lines)
    {
        var pivot = FindPalindromePivot(lines);
        if (pivot > 0)
            return pivot;

        pivot = FindPalindromePivot(Rotate(lines));
        if (pivot > 0)
            return 100 * pivot;

        return 0;
    }

    private static int FindPalindromePivot(char[][] lines)
    {
        var pivots = new List<int>();

        foreach (var line in lines)
        {
            var p = FindPalindromePivots(line);
            pivots.AddRange(p);
        }

        return pivots
            .GroupBy(p => p)
            .FirstOrDefault(g => g.Count() == lines.Length - 1)?
            .Key ?? 0;
    }

    private static ISet<int> FindPalindromePivots(char[] chars)
    {
        var pivots = new HashSet<int>();

        for (var p = 1; p < chars.Length; p++)
        {
            var left = chars.Take(p).Reverse().ToArray();
            var right = chars.Skip(p).ToArray();
            var length = Math.Min(left.Length, right.Length);

            if (left.Take(length).SequenceEqual(right.Take(length)))
                pivots.Add(p);
        }

        return pivots;
    }

    private static char[][] Rotate(char[][] chars)
    {
        var result = new char[chars[0].Length][];

        for (var i = 0; i < result.Length; i++)
        {
            result[i] = new char[chars.Length];

            for (var j = 0; j < result[i].Length; j++)
                result[i][j] = chars[j][i];
        }

        return result;
    }
}
