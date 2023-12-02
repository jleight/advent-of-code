namespace AoC.Y2017.Solutions;

public class D02P2 : ISolution
{
    public Task Run(SolutionContext context)
    {
        var input = context.InputLines
            .Select(l => l.Split("\t").Select(int.Parse).ToArray())
            .ToArray();

        var sum = 0;
        foreach (var line in input)
        {
            for (var a = 0; a < input.Length; a++)
            for (var b = 0; b < input.Length; b++)
            {
                if (a == b || line[a] % line[b] > 0)
                    continue;
                sum += line[a] / line[b];
            }
        }

        Console.WriteLine(sum);
        return Task.CompletedTask;
    }
}
