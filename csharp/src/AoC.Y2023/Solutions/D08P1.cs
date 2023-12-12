namespace AoC.Y2023.Solutions;

public class D08P1 : ISolution
{
    public Task Run(SolutionContext context)
    {
        using var moves = context.InputLines[0]
            .ToArray()
            .Repeat()
            .GetEnumerator();

        var maps = context.InputLines[2..]
            .ToDictionary(
                l => l[..3],
                l => (Left: l[7..10], Right: l[12..15]));

        var location = "AAA";
        var move = 0;

        while (location != "ZZZ")
        {
            moves.MoveNext();

            location = moves.Current == 'L'
                ? maps[location].Left
                : maps[location].Right;
            move += 1;
        }

        Console.WriteLine(move);
        return Task.CompletedTask;
    }
}
