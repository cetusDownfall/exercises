package cellularAutomata.utils;
/**	An interface to create classes to serve as the value of a cellular point.
  *	Reqs: a static method to return a default state (initial field value) and a getter and incrementer for state and getset for grid.
  *	Note that T, an enumerated type, is the list of states. These should have color values.
  *	Ryugo Okada
  *	11/30/2017
  */
import cellularAutomata.utils.Grid;
public interface State<T extends Enum>{
	static T getDefaultState();
	T getState();
	void setState(T next);
	T getColor();
}
