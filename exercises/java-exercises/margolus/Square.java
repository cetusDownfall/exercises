package cellularAutomata.margolus;

/**	Holds 4 values and returns them in order.
  *	Cetus
  *	11/30/17
  */
import java.util.Arrays;
import java.util.ArrayList;
import java.util.List;
import java.util.*;

public class Square<T>{
	private T[] box = new T[4];
	private int[] pos = new int[2];	//references the upper left corner of the box.
	public Square(int posr, int posc, T...corners){
		for(int i = 0; i < 4; i++){
			box[i] = corners[i];
		}
		pos[0] = posr;
		pos[1] = posc;
	}
	public void set(){
		
