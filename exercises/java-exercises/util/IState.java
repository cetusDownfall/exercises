package cellularAutomata.utils;
public interface IState<T>{
	T getNextState();
	T getCurrentState();
}
