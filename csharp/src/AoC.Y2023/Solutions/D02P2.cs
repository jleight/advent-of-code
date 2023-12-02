namespace AoC.Y2023.Solutions;

public class D02P2 : ISolution
{
    public Task Run(SolutionContext context)
    {
        var sum = context
            .InputLines
            .Select(Game.Parse)
            .Select(CalculatePower)
            .Sum();

        Console.WriteLine(sum);
        return Task.CompletedTask;

        int CalculatePower(Game game)
        {
            return game.Rounds.Max(r => r.Cubes.GetValueOrDefault("red")) *
                   game.Rounds.Max(r => r.Cubes.GetValueOrDefault("green")) *
                   game.Rounds.Max(r => r.Cubes.GetValueOrDefault("blue"));
        }
    }

    private record Game(
        ICollection<GameRound> Rounds)
    {
        public static Game Parse(string line)
        {
            var split = line.Split(":", StringSplitOptions.TrimEntries);

            var rounds = split
                .Last()
                .Split(";", StringSplitOptions.TrimEntries)
                .Select(GameRound.Parse)
                .ToArray();

            return new Game(rounds);
        }
    }

    private record GameRound(
        Dictionary<string, int> Cubes)
    {
        public static GameRound Parse(string line)
        {
            var cubes = line
                .Split(",", StringSplitOptions.TrimEntries)
                .Select(c => c.Split(" ", StringSplitOptions.TrimEntries))
                .ToDictionary(
                    c => c.Last(),
                    c => c.First().ConvertTo<int>());

            return new GameRound(cubes);
        }
    }
}
