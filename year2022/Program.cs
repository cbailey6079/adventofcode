using System.Reflection;

namespace year_2022
{
    class Program
    {
        static void Main()
        {
            Console.WriteLine("Welcome to 2022 - Advent Of Code!");
            Console.WriteLine("You will only finish about 12 of these so don't fool yourself.");

            Thread.Sleep(1000);
            
            Problems.SelectProblem();
        }
    }

    class Problems
    {
        public static void SelectProblem()
        {
            int lastSolved = 3;
            int[] days = Enumerable.Range(1, lastSolved).ToArray();

            input:
            Console.WriteLine("\nSelect one of the follow days to run:");
            foreach (var day in days)
            {
                Console.WriteLine(day);
            }
            Console.WriteLine("--------------------------");

            string? dayToRun = Console.ReadLine();

            if (Int32.TryParse(dayToRun, out var x) && days.Contains(x))
            {
                SelectPart(x);
                goto input;
            }
            else if (dayToRun == "q")
            {
                Console.Beep();
            }
            else
            {
                goto input;
            }
        }

        static void SelectPart(int day)
        {
            partinput:
            Console.WriteLine("Run Part 1 or 2");
            string? partToRun = Console.ReadLine();

            if (Int32.TryParse(partToRun, out var part) || partToRun == "test")
            {
                Console.WriteLine("Running....");
                
                RunProgram(day, part != 0 ? part.ToString() : partToRun);
            }
            else if (partToRun == "q")
            {
                Console.Beep();
            }
            else
            {
                goto partinput;
            }
        }

        static void RunProgram(int day, string part)
        {
            Type type = Type.GetType("year_2022.Days.Day" + day)!;
            Object obj = Activator.CreateInstance(type)!;
            MethodInfo methodInfo = type.GetMethod("Part" + part)!;

            Console.WriteLine("==================== Day {0} Part {1} Answer ====================", day, part);
            Console.WriteLine(methodInfo.Invoke(obj, null));
        }
    }
}


