namespace AoC.Y2017.Solutions;

public class D02P1 : ISolution
{
    public Task Run(SolutionContext context)
    {
        var input = context.InputLines
            .Select(l => l.Split("\t").Select(n => Convert.ToInt32(n, 10)).ToArray())
            .ToArray();

        var sum = 0;
        foreach (var line in input)
        {
            var max = int.MinValue;
            var min = int.MaxValue;
            foreach (var number in line)
            {
                max = Math.Max(max, number);
                min = Math.Min(min, number);
            }

            sum += max - min;
        }

        Console.WriteLine(sum);
        return Task.CompletedTask;
    }
}
