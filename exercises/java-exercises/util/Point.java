package cellularAutomata.utils;

import java.util.ArrayList;
import java.util.*;
import java.util.Arrays;
//A simple data type class for holding x y coords and any number of state integers. Uses arrays.

public class Point{
	private int[] pos = new int[2];
	//Params: x y coords, state.
	public Point(int x, int y){
		pos[0] = x;
		pos[1] = y;
	}
	//Params: none
	//Returns: x y coords in an array. 0 is x, 1 is y.
	public int[] getPos(){
		return pos;
	}
	public int getPos(int i){
		return pos[i];
	}
	public Point toPoint(){
		return (new Point(getPos(0), getPos(1)));
	}
	public void translate(int i, int m){
		pos[i] += m;
	}
	public void setPos(int i, int n){
		pos[i] = n;
	}
	public static Point center(int d){
		return new Point(d/2, d/2);
	}
	//Params: two point instances
	//returns: boolean
	public static boolean posEquals(Point a, Point b){
		return (Arrays.equals(a.getPos(), b.getPos()));
	}
}
