package cellularAutomata.utils;
import cellularAutomata.*;
import cellularAutomata.langton.Ant;
import java.util.ArrayList;
import java.util.*;
public class RemDupes{
	public static ArrayList<Ant> remAnts(ArrayList<Ant> ants){
		ArrayList<Ant> out = new ArrayList<Ant>();
		for (Ant a : ants){
			boolean hasDup = false;
			for (Ant n : out){
				if (n.equals(a)){
					hasDup = true;
				}
			}
			if (!hasDup){
				out.add(a);
			}
		}
		return out;
	}
}

