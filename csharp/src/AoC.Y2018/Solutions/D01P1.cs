namespace AoC.Y2018.Solutions;

[UsedImplicitly]
public class D01P1 : ISolution
{
    public Task Run(SolutionContext context)
    {
        var result = context.InputLines
            .Select(e => e.Trim().TrimStart('+'))
            .Select(e => Convert.ToInt64(e, 10))
            .Aggregate((a, e) => a + e);

        Console.WriteLine(result);
        return Task.CompletedTask;
    }
}
