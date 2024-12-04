namespace AoC.Y2024.Solutions;

public class D02P1 : ISolution
{
    public Task Run(SolutionContext context)
    {
        var safe = context
            .InputLines
            .Select(i => i
                .Split(' ', StringSplitOptions.RemoveEmptyEntries)
                .Select(l => l.ConvertTo<int>())
                .ToArray())
            .Select(Test)
            .Select(Convert.ToInt32)
            .Sum();

        Console.WriteLine(safe);
        return Task.CompletedTask;

        bool Test(int[] levels)
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

            return differences
                .All(d => d.Abs is >= 1 and <= 3 && d.Sign == sign);
        }
    }
}
