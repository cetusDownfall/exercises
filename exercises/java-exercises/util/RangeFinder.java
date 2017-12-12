package cellularAutomata.utils;
import cellularAutomata.langton.*;
public class RangeFinder{
	private Range row, col;
	public RangeFinder(Point p1, Point p2){
		this(p1.getPos(0), p2.getPos(0), p1.getPos(1), p2.getPos(1));
	}
	public RangeFinder(int ur, int lr, int uc, int lc){
		row = new Range(ur, lr);
		col = new Range(uc, lc);
	}
	public boolean contains(Ant a){
		return this.contains(a.toPoint());
	}
	public boolean contains(Point p){
		return (row.contains(p.getPos(0)) && col.contains(p.getPos(1)));
	}
	public boolean contains(Point p, int i){
		boolean bout = true;
		switch(i){
			case 0: bout = row.contains(p.getPos(0));break;
			case 1: bout = col.contains(p.getPos(1));break;
			default: bout = contains(p);break;
		}
		return bout;
	}
	public Range get(int i){
		return i==0?row:col;
	}
	public boolean onEdge(Point p){
		return row.edge(p.getPos(0))==0||col.edge(p.getPos(1))==0;
	}
}
