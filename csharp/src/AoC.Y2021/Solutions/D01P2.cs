namespace AoC.Y2021.Solutions;

[UsedImplicitly]
public class D01P2 : ISolution
{
    private const int Window = 3;

    public Task Run(SolutionContext context)
    {
        var input = context.InputLines
            .Select(int.Parse)
            .ToArray();

        var last = int.MaxValue;
        var increases = 0;
        for (var i = 0; i < input.Length - (Window - 1); i++)
        {
            var next = input[i..(i + Window)].Sum();
            if (next > last)
                increases += 1;
            last = next;
        }

        Console.WriteLine(increases);
        return Task.CompletedTask;
    }
}
