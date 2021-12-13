namespace AoC.Y2021.Solutions;

[UsedImplicitly]
public class D01P1 : ISolution
{
    public Task Run(SolutionContext context)
    {
        var input = context.InputLines
            .Select(int.Parse)
            .ToArray();

        var last = int.MaxValue;
        var increases = 0;
        foreach (var i in input)
        {
            if (i > last)
                increases += 1;
            last = i;
        }

        Console.WriteLine(increases);
        return Task.CompletedTask;
    }
}
