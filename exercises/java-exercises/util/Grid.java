package cellularAutomata.utils;
/**	A modular grid to contain instances of any type which implements the cellularpoint interface.
  *	Cetus
  *	11/30/2017
  */
import java.util.Arrays;
import java.util.List;
import java.util.Set;
import java.util.ArrayList;
import java.util.HashSet;

public abstract class Grid<T extends CellularPoint>{
	private Set<T> contents = new HashSet<T>();
	private long step = 0;
	public Grid(int size){
		if(T.getDegree()>2){
		}else{
			while(contents.size()!=Math.pos(T.getDegree())){
				T tmp = new T();
				T.setGrid(this);
			}
		}
	}
}
