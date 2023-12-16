namespace AoC.Y2023.Solutions;

public class D12P2 : ISolution
{
    public Task Run(SolutionContext context)
    {
        var sum = context
            .InputLines
            .Select(l => l.Split(' '))
            .Select(l => new[]
            {
                string.Join('?', Enumerable.Repeat(l[0], 5)),
                string.Join(',', Enumerable.Repeat(l[1], 5))
            })
            .Select(l => CalculatePossibilities(
                $"{l[0]}.",
                l[1].Split(',').Select(int.Parse).ToArray()))
            .Sum();

        Console.WriteLine(sum);
        return Task.CompletedTask;
    }

    private static decimal CalculatePossibilities(string springs, int[] groups)
    {
        var total = 0m;

        var states = new Dictionary<State, decimal> { [new State(0, 0, 0)] = 1 };
        var nextStates = new Dictionary<State, decimal>();

        while (states.Count > 0)
        {
            foreach (var (state, duplicates) in states)
            {
                var (springIndex, groupIndex, _) = state;

                // if we've made it through all the springs, we're done testing this possibility
                if (springIndex == springs.Length)
                {
                    // if we've made it through all the groups, this was a valid solution
                    if (groupIndex == groups.Length)
                        total += duplicates;

                    // otherwise, this isn't a valid solution
                    continue;
                }

                // if we've made it through all the groups, but haven't made it through all the springs, then
                // this isn't a valid solution
                if (groupIndex > groups.Length)
                    continue;

                var spring = springs[springIndex];
                var group = groupIndex == groups.Length
                    ? 0
                    : groups[groupIndex];

                if (spring is '.' or '?')
                    HandleWorkingSpring(nextStates, state, duplicates, group);
                if (spring is '#' or '?')
                    HandleBrokenSpring(nextStates, state, duplicates, group);
            }

            states = nextStates;
            nextStates = new();
        }

        return total;
    }

    private static void HandleWorkingSpring(
        IDictionary<State, decimal> next,
        State state,
        decimal duplicates,
        int expectedCount)
    {
        var (_, groupIndex, groupCount) = state;

        // if we haven't found any broken springs yet
        if (groupCount == 0)
        {
            // just skip this spring
            Increment(next, state.Next(groupIndex, 0), duplicates);
            return;
        }

        // otherwise, we've found the end of a group of broken springs. if we haven't seen
        // the expected number of broken springs, then this isn't a valid solution
        if (groupCount != expectedCount)
            return;

        // otherwise, we move on to the next group
        Increment(next, state.Next(groupIndex + 1, 0), duplicates);
    }

    private static void HandleBrokenSpring(
        IDictionary<State, decimal> next,
        State state,
        decimal duplicates,
        int expectedCount)
    {
        var (_, groupIndex, groupCount) = state;

        // if we've already found too many broken springs in this group, then this isn't a
        // valid solution
        if (groupCount >= expectedCount)
            return;

        // otherwise, move on to the next spring and increment the count for this group
        Increment(next, state.Next(groupIndex, groupCount + 1), duplicates);
    }

    private static void Increment(
        IDictionary<State, decimal> states,
        State state,
        decimal value)
    {
        states[state] = states.TryGetValue(state, out var existing)
            ? existing + value
            : value;
    }

    private record State(
        int SpringIndex,
        int GroupIndex,
        int GroupCount)
    {
        public State Next(int groupIndex, int groupCount)
            => new(SpringIndex + 1, groupIndex, groupCount);
    }
}
