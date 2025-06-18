#include <algorithm>
#include<iostream>
#include<vector>

using namespace std;

class prioQueue {
public:
    int size;
    pair<int,int>* array;
    prioQueue() {
        array = new pair<int,int>[1000010];
        fill_n(array,1000010,make_pair(0,0));
        size = 0;
    }
private :
    long long value(int index) {
        return (long long)array[index].first*(long long)array[index].second;
    }
    void move_up(int index) {
        if ( index <= 1) return;
        if (value(index) > value(index/2)) {
            swap(array[index], array[index/2]);
            return move_up(index/2);
        }
    }
    void move_down(int index) {
        if ( 2*index > size) return;
        int j = index;
        if (value(2*index) > value(j)) {j = 2*index;}
        if (value(2*index+1) > value(j)) {j = 2*index+1;}
        swap(array[j], array[index]);
        if (j!=index) {
            return move_down(j);
        }

    }
public:
    void insert(pair<int, int> elem) {
        size++;
        array[size] = elem;
        move_up(size);
    }
    void pop() {
        if (size == 0) return;
        swap(array[1], array[size]);
        array[size] = make_pair(-1,0);
        size--;
        move_down(1);
    }
    pair<int,int> maximum() {return array[1];}
};

long long curr_res = -1;

int main() {
    ios_base::sync_with_stdio(false);
    auto q = prioQueue();
    int m,k;
    cin>>m>>k;

    for(int i = 1; i <= m; i++) {
        q.insert({m,i});
    }

    for(int i = 0; i < k; i++) {
        auto [mult, col] = q.maximum();
        long long curr_max = (long long)mult*(long long)col;
        q.pop();
        if(mult > col) {
            mult--;
            q.insert({mult,col});
        }
        if(curr_max == curr_res) i--;
        else {
            curr_res = curr_max;
            cout<<curr_res<<"\n";
        }
    }

    return 0;


}