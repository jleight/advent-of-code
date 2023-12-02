namespace AoC.Y2022.Solutions;

public class D04P1 : ISolution
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
            .Count(p => p.A.Contains(p.B) || p.B.Contains(p.A));

        Console.WriteLine(result);
        return Task.CompletedTask;
    }
}

file record Range(int Start, int End)
{
    public bool Contains(Range other)
        => Start <= other.Start && End >= other.End;
}
