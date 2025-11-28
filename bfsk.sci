function [y]= wsqr(yin)
    count=1;
    y=yin;
    for i=yin;
        if(i>=0)
            y(count)=1;
        else
            y(count)=0;
        end
        count=count+1
    end
        
endfunction

function [y]=bpsk(w1,w2,dserial)
    y=w1;
    count=1;
    for i=dserial;
        if(i>0)   //This is the selector
            y(count)=w2(count);
        else
            y(count)=w1(count);
        end
        count=count+1
    end
    
endfunction


t=0:0.25:10;
frequency=1; //Hz
frequency1=1/16; //Hz
phase0=0;
phase1=%pi;
f0=((t*%pi*2))*frequency;
f1=((t*%pi*2))*frequency1;
y0=sin(f0);
y1=sin(f0+phase1);
y3=sin(f1);
ysqr=wsqr(y3);
ybpsk=bpsk(y0,y1,ysqr);


plot(t,ybpsk)
