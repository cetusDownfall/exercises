package cellularAutomata.margolus;

/**	Class to hold relevant information of an array and return two arrays which alternate depending on the step.
  *	Cetus 11/30/2017
  */
import java.util.Arrays;
import java.util.List;
import java.util.ArrayList;
import java.util.Map;
import java.util.HashMap;
import java.util.*;

public class LatticeAlternator<T>{
	private T[] contents;
	private List<Square<T>> currStep = new ArrayList<T>();
	private int step;
