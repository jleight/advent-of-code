namespace AoC.Y2024.Solutions;

public class D02P2 : ISolution
{
    public Task Run(SolutionContext context)
    {
        var safe = context
            .InputLines
            .Select(i => i
                .Split(' ', StringSplitOptions.RemoveEmptyEntries)
                .Select(l => l.ConvertTo<int>())
                .ToArray())
            .Select(l => Test(l, true))
            .Select(Convert.ToInt32)
            .Sum();

        Console.WriteLine(safe);
        return Task.CompletedTask;

        bool Test(
            int[] levels,
            bool dampen)
        {
            var differences = levels
                .Zip(levels.Skip(1), Tuple.Create)
                .Select(p => p.Item1 - p.Item2)
                .Select(d => (Abs: Math.Abs(d), Sign: Math.Sign(d)))
                .ToArray();

            var sign = differences
                .GroupBy(d => d.Sign)
                .OrderByDescending(g => g.Count())
                .First()
                .Key;

            if (differences.All(d => d.Abs is >= 1 and <= 3 && d.Sign == sign))
                return true;

            if (!dampen)
                return false;

            return levels
                .Select((_, i) => levels
                    .Where((_, j) => j != i)
                    .ToArray())
                .Any(l => Test(l, false));
        }
    }
}
