package cellularAutomata.utils;

public class ModularInteger extends Range{
        private int value;
	public ModularInteger(int iv, int ru){
		super(ru, -1);
		value = iv;
		refresh();
	}
	public	void refresh(){
		if (!contains(value)){
			value = (0 + value%getUpper() + getUpper())%getUpper();
		}
	}
	public int getValue(){
		refresh();
		return value;
	}
	public void add(int n){
		value += n;
		refresh();
	}
	public void multiply(int n){
		value *= n;
		refresh();
	}
	public void divide(int n){
		value /= n;
		refresh();
	}
	public void set(int n){
		value = n;
		refresh();
	}
	public ModularInteger immutAdd(int n){
		return new ModularInteger(value + n, getUpper());
	}
	public ModularInteger immutMultiply(int n){
		return new ModularInteger(value * n, getUpper());
	}
	public ModularInteger immutDivid(int n){
		return new ModularInteger(value/n, getUpper());
	}
}
