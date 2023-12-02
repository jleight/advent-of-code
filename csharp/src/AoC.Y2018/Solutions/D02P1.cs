namespace AoC.Y2018.Solutions;

public class D02P1 : ISolution
{
    public Task Run(SolutionContext context)
    {
        var (doubles, triples) = context.InputLines
            .Select(Parse)
            .Aggregate((a, e) => (a.Doubles + e.Doubles, a.Triples + e.Triples));

        Console.WriteLine(doubles * triples);
        return Task.CompletedTask;
    }

    private static (int Doubles, int Triples) Parse(string barcode)
    {
        var counts = barcode
            .GroupBy(c => c)
            .Select(g => g.Count())
            .Distinct()
            .ToList();
        var doubles = counts.Count(c => c == 2);
        var triples = counts.Count(c => c == 3);
        return (doubles, triples);
    }
}
