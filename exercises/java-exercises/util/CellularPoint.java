package cellularAutomata.utils;
/**	An interface to be used to specify the behavior of a point in a cellular automata.
  *	Requires: a getter and setter for pos, grid, and state. Also a void method to transition to the next state.
  *	Cetus
  *	11/30/2017
  */
import cellularAutomata.utils.State;
import cellularAutomata.utils.Grid;
import cellularAutomata.utils.NDimPoint;
public abstract class CellularPoint<T extends State> extends NDimPoint{
	public CellularPoint(int posr, int posc, T state){
		setPos()
	void setPoint(int...pos);
	V getState();
	void nextState();
	void setstate(V st);
	Grid getGrid();
	void setGrid();
	static V getDefaultState();
}
