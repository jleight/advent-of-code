namespace AoC.Y2022.Solutions;

[UsedImplicitly]
public class D04P2 : ISolution
{
    public Task Run(SolutionContext context)
    {
        var result = context.InputLines
            .Select(l => l.Split(","))
            .Select(p => new
            {
                A = p[0].Split("-"),
                B = p[1].Split("-")
            })
            .Select(p => new
            {
                A = new Range(int.Parse(p.A[0]), int.Parse(p.A[1])),
                B = new Range(int.Parse(p.B[0]), int.Parse(p.B[1]))
            })
            .Count(p => p.A.Overlaps(p.B) || p.B.Overlaps(p.A));

        Console.WriteLine(result);
        return Task.CompletedTask;
    }
}

file record Range(int Start, int End)
{
    public bool Overlaps(Range other)
        => other.Start <= Start && other.End >= Start;
}