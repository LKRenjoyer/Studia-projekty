#include <iostream>
#include <tuple>
using namespace std;

class AVL_Tree {
    const int LEFT = 0, RIGHT = 1;
    static constexpr int SIZE = 0, SUM = 1, VAL = 2;
    class Node {
    public:
        tuple<int,long long,long long> data;
        int height;
        Node* left;
        Node* right;
        explicit Node(long long val) {
            height = 1;
            left = nullptr;
            right = nullptr;
            data = make_tuple(1,val,val);
        }

    };
public:
    Node* root = nullptr;
private:
    int height(const Node* node) {
        if (node == nullptr)return 0;
        return node->height;
    }
    int balance(const Node* node) {
        if (node == nullptr)return 0;
        return height(node->left) - height(node->right);
    }
    static int size(const Node* node) {
        if (node == nullptr)return 0;
        return get<SIZE>(node->data);
    }
    static long long sum(const Node* node) {
        if (node == nullptr)return 0;
        return get<SUM>(node->data);
    }
    static long long val(const Node* node) {
        if (node == nullptr)return 0;
        return get<VAL>(node->data);
    }
    void recalc(Node* node) {
        long long new_sum = sum(node->left) + sum(node->right) + val(node);
        const int new_height = 1 + max(height(node->left), height(node->right));
        const int new_size = 1 + size(node->left) + size(node->right);
        node -> data = tuple<int,long long, long long>(new_size,new_sum,val(node));
        node -> height = new_height;
    }


private:
    Node* pred(Node* node) {
        if (node == nullptr)return nullptr;
        if (node->right == nullptr) return node;
        return pred(node->right);
    }
    Node* rotate(Node* parent, const int rot_type) {
        Node* new_root = nullptr;
        if (rot_type == RIGHT) {
            new_root = parent->left;
            Node* beta = new_root->right;
            parent->left = beta;
            new_root->right = parent;
        }
        else {
            new_root = parent->right;
            Node* beta = new_root->left;
            parent->right = beta;
            new_root->left = parent;
        }

        recalc(parent);
        recalc(new_root);
        return new_root;
    }
private:
    Node* insert_helper(Node* curr_root, long long val, int p) {
        if (curr_root == nullptr) {
            curr_root = new Node(val);
            return curr_root;
        }
        if (p <= size(curr_root->left))curr_root->left = insert_helper(curr_root->left,val,p);
        else curr_root->right = insert_helper(curr_root->right,val,p-size(curr_root->left)-1);
        recalc(curr_root);
        switch (balance(curr_root)) {
            case 2 :
                if (balance(curr_root->left) < 0)
                    curr_root->left = rotate(curr_root->left, LEFT);
                return rotate(curr_root,RIGHT);
            break;
            case -2 :
                if (balance(curr_root->right) > 0)
                    curr_root->right = rotate(curr_root->right,RIGHT);
                return rotate(curr_root,LEFT);
            break;
            default: ;
        }
        return curr_root;
    }
    Node* delete_helper(Node* curr_root, int p) {
        if (curr_root == nullptr) return nullptr;

        if (p <= size(curr_root->left))  curr_root->left = delete_helper(curr_root->left,p);
        else if (p> size(curr_root->left)+1) curr_root->right = delete_helper(curr_root->right,p-size(curr_root->left)-1);
        else {
            if (curr_root->left == nullptr && curr_root->right == nullptr) {
                delete curr_root;
                return nullptr;
            } else if (curr_root->right&& curr_root->left) {
                get<VAL>(curr_root->data) = val(pred(curr_root->left));
                curr_root->left = delete_helper(curr_root->left,size(curr_root->left));
                recalc(curr_root);
            }
            else {
                Node* to_del = curr_root;
                if (curr_root->left != nullptr) curr_root = curr_root->left;
                else curr_root = curr_root->right;
                to_del->left = nullptr;
                to_del->right = nullptr;
                delete to_del;
                to_del = nullptr;
            }
        }
        recalc(curr_root);
        switch (balance(curr_root)) {
            case 2 :
                if (balance(curr_root->left) < 0)
                    curr_root->left = rotate(curr_root->left, LEFT);
            return rotate(curr_root,RIGHT);
            break;
            case -2 :
                if (balance(curr_root->right) > 0)
                    curr_root->right = rotate(curr_root->right,RIGHT);
            return rotate(curr_root,LEFT);
            break;
            default: ;
        }
        return curr_root;
    }
    long long query_helper(Node* curr_root, int p1, int p2) {
        if (p1 == 1 && p2 >= size(curr_root)) return sum(curr_root);
        long long lef=0,mid=0,rig=0;
        if (p1<=size(curr_root->left)+1 && p2>=size(curr_root->left)+1) mid = val(curr_root);
        if (p1<=size(curr_root->left)) lef = query_helper(curr_root->left,p1,p2);
        if (p2>size(curr_root->left)+1) rig = query_helper(curr_root->right,max(1,p1-size(curr_root->left)-1),p2-size(curr_root->left)-1);
        return lef+mid+rig;
    }
public :
    void insert(long long val, int p) {
        this->root = insert_helper(root,val,p);
    }
    void delete_elem(int p) {
        this->root = delete_helper(root,p);
    }
    long long query(int p1, int p2) {
        return query_helper(root,p1,p2);
    }

};


int main() {
    ios_base::sync_with_stdio(false);
    cin.tie(nullptr);
    cout.tie(nullptr);
    int n;
    AVL_Tree tree;
    cin>>n;

    while (n--) {
        char command;
        cin>>command;
        switch (command) {
            long long x;
            int p,p1,p2;
            case 'I' :
                cin>>p>>x;
                tree.insert(x,p);
                break;
            case 'D' :
                cin>>p;
                tree.delete_elem(p);
                break;
            case 'S' :
                cin>>p1>>p2;
                cout<<tree.query(p1,p2)<<'\n';
                break;
            default : ;
        }
    }

    return 0;
}