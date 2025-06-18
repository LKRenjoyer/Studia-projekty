#include <iostream>
using namespace std;
int m1,m2;
string goal;
int type1[8][8] = {}, type2[26];
int obtainable[1000][1000];
int cache[256][256];
void clear() {
    for ( int i=0;i<8;i++) {
        for ( int j=0;j<8;j++) {
            type1[i][j] = 0;
        }
    }
    for (int i=0;i<26;i++) type2[i] = 0;
}

void init_row() {
    for (int i=0;i<goal.size();i++) {
        obtainable[i][i] = type2[goal[i] - 'a'];
    }
}
void calc_cache(const int lef, const int rig) {
    int res = 0;
    for (int i=0;i<8;i++) {
        if (lef & (1<<i)) {
            for (int j=0;j<8;j++) {
                if (rig & (1<<j)) {
                    res |= type1[i][j];
                }
            }
        }
    }
    cache[lef][rig] = res;
}
void calc_a_cell(const int st, const int en) {
    int res = 0;
     for (int ptr = st; ptr < en; ptr++) {
         const int lef = obtainable[st][ptr];
         const int rig = obtainable[ptr+1][en];
         res |= cache[lef][rig];
     }
    obtainable[st][en] = res;
}
void calc_a_row(const int len) {
    for (int en = len-1; en < goal.size(); en ++) {
        calc_a_cell(en-len+1,en);
    }
}
bool solve() {
    clear();
    cin>>m1>>m2;
    for (int i=0;i<m1;i++) {
        char a,b,c;
        cin>>a>>b>>c;
        type1[b-'A'][c-'A'] |= 1<<(a-'A');
    }
    for (int i=0;i<m2;i++) {
        char a,b;
        cin>>a>>b;
        type2[b-'a'] |= 1<<(a-'A');
    }
    cin>>goal;
    init_row();
    for (int i=0;i<256;i++) {
        for (int j=0;j<256;j++) {
            calc_cache(i,j);
        }
    }
    for (int len=2;len <= goal.size(); len++)calc_a_row(len);
    return obtainable[0][goal.size()-1]%2;
}


int main() {
    ios_base::sync_with_stdio(false);
    cin.tie(nullptr);
    int n;
    cin>>n;
    while (n--) {
        if (solve()) {
            cout<<"TAK"<<endl;
        } else {
            cout<<"NIE"<<endl;
        }
    }
}